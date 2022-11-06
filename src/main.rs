// use comfy_table::Table; TODO for printing our tables, cols etc.
use std::collections::HashMap;
use std::fs::{self, ReadDir};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static PKG_NAME: &str = env!("CARGO_PKG_NAME");

static LEXER_SEPARATOR: [char; 5] = [' ', ',', '\n', '\t', ';'];
static LEXER_SKIP: [&'static str; 5] = ["", ",", ";", "\n", "\t"];

#[derive(Debug, Eq, Hash, PartialEq, EnumIter)]
enum SqlStatement {
    Select,
    Insert,
    Update,
    Delete,
    Create,
    Drop,
    Alter,
    Unknown,
}

fn lex_file(file_name: &str) -> Vec<String> {
    // NOTE no analysis for now
    let file: String = fs::read_to_string(file_name).expect("Unable to parse '{file_name}'");

    let tokens: Vec<String> = file
        .to_lowercase()
        .split(|s| LEXER_SEPARATOR.contains(&s))
        .map(|s| s.to_string())
        .collect();

    tokens
        .iter()
        .filter(|token| LEXER_SKIP.contains(&token.as_str()) == false)
        .cloned()
        .collect()
}

fn get_statement_type(tokens: &Vec<String>) -> SqlStatement {
    let statement_type = match tokens[0].as_str() {
        "select" => SqlStatement::Select,
        "insert" => SqlStatement::Insert,
        "update" => SqlStatement::Update,
        "delete" => SqlStatement::Delete,
        "create" => SqlStatement::Create,
        "drop" => SqlStatement::Drop,
        "alter" => SqlStatement::Alter,
        _ => SqlStatement::Unknown,
    };

    statement_type
}

fn init_statements_map() -> HashMap<SqlStatement, Vec<String>> {
    let mut statements_map: HashMap<SqlStatement, Vec<String>> = HashMap::new();

    for statement_type in SqlStatement::iter() {
        statements_map.insert(statement_type, Vec::new());
    }

    statements_map
}

fn populate_statements_for(path: &str, statements_map: &mut HashMap<SqlStatement, Vec<String>>) {
    let files = fs::read_dir(path).expect("Unable to read current directory");

    for file in files {
        let file_name = file.unwrap().path().display().to_string();
        if file_name.ends_with(".sql") {
            debug!("parsing file '{}'", file_name);

            let tokens = lex_file(&file_name);
            let statement_type = get_statement_type(&tokens);

            debug!(
                "file: '{}' statement type: '{:?}'",
                file_name, statement_type
            );

            statements_map
                .get_mut(&statement_type)
                .unwrap()
                .push(file_name);
        }
    }
}

fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    info!("running '{}' with version '{}'", PKG_NAME, VERSION);

    let mut sql_statements = init_statements_map();

    populate_statements_for("src/test/data/", &mut sql_statements);

    info!("-- ------------------------------------------");
    for (statement_type, files) in sql_statements {
        debug!(
            "'{:?}' len: '{}' files: '{}'",
            statement_type,
            files.len(),
            files.join(", ")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_with_cols() {
        let tokens_with_cols = lex_file("src/test/data/select_with_cols.sql");

        assert_eq!(
            tokens_with_cols,
            vec![
                "select",
                "*",
                "title",
                "platforms",
                "released",
                "from",
                "video_games"
            ]
        );
        assert_eq!(get_statement_type(&tokens_with_cols), SqlStatement::Select);
    }

    #[test]
    fn test_insert_vg() {
        let tokens_insert_vg = lex_file("src/test/data/insert_vg.sql");
        assert_eq!(get_statement_type(&tokens_insert_vg), SqlStatement::Insert);
    }

    #[test]
    fn test_populate_statements_map() {
        let mut statements_map = init_statements_map();

        populate_statements_for("src/test/data/", &mut statements_map);

        assert_eq!(statements_map.len(), 8);
        assert_eq!(statements_map.get(&SqlStatement::Select).unwrap().len(), 2);
        assert_eq!(statements_map.get(&SqlStatement::Insert).unwrap().len(), 1);
    }
}

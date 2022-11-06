use comfy_table::Table;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static PKG_NAME: &str = env!("CARGO_PKG_NAME");

static LEXER_SEPARATOR: [char; 5] = [' ', ',', '\n', '\t', ';'];
static LEXER_SKIP: [&'static str; 5] = ["", ",", ";", "\n", "\t"];

static TABLE_HEADERS: [&str; 4] = ["FILE", "STATEMENT_TYPE", "TABLES", "USED_BY"];

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

fn populate_maps_for(
    path: &str,
    statements_map: &mut HashMap<SqlStatement, Vec<String>>,
    table_map: &mut HashMap<String, Vec<String>>,
) {
    let files = fs::read_dir(path).expect("Unable to read current directory");

    for file in files {
        let file_name = file.unwrap().path().display().to_string();
        if file_name.ends_with(".sql") {
            let file_name_trimmed = trim_prefix_from(&file_name);

            let tokens = lex_file(&file_name);
            let statement_type = get_statement_type(&tokens);

            statements_map
                .get_mut(&statement_type)
                .unwrap()
                .push(file_name_trimmed.to_owned());

            let tables = parse_tables_from_tokens(&statement_type, &tokens);
            table_map.insert(file_name_trimmed, tables);
        }
    }
}

fn trim_prefix_from(file_name: &str) -> String {
    let file_name_trimmed = Path::new(&file_name)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    file_name_trimmed
}

fn parse_tables_from_tokens(statement_type: &SqlStatement, tokens: &Vec<String>) -> Vec<String> {
    let mut tables: Vec<String> = Vec::new();

    match statement_type {
        SqlStatement::Select => {
            tokens.iter().enumerate().for_each(|(i, token)| {
                if token == "from" {
                    tables.push(tokens[i + 1].to_owned());
                }
            });
        }
        SqlStatement::Insert => {
            tokens.iter().enumerate().for_each(|(i, token)| {
                if token == "insert" {
                    tables.push(tokens[i + 2].to_owned());
                }
            });
        }
        SqlStatement::Update => {
            // TODO
        }
        SqlStatement::Delete => {
            // TODO
        }
        SqlStatement::Create => {
            // TODO
        }
        SqlStatement::Drop => {
            // TODO
        }
        SqlStatement::Alter => {
            // TODO
        }
        SqlStatement::Unknown => {
            // TODO
        }
    }

    tables
}

fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    info!("running '{}' with version '{}'", PKG_NAME, VERSION);

    let mut sql_statements = init_statements_map();
    let mut table_map: HashMap<String, Vec<String>> = HashMap::new();

    populate_maps_for("src/test/data/", &mut sql_statements, &mut table_map);

    info!("-- ------------------------------------------");

    // TODO define relationship seqs: insert -> select

    let mut out = Table::new();
    out.set_header(TABLE_HEADERS);

    for (statement_type, insert_files) in &sql_statements {
        if *statement_type == SqlStatement::Insert {
            for file in insert_files {
                let insert_tables = table_map.get(file).unwrap();
                for table in insert_tables {
                    let select_files = sql_statements.get(&SqlStatement::Select).unwrap();

                    let mut used: Vec<String> = Vec::new();
                    for select_file in select_files {
                        let select_tables = table_map.get(select_file).unwrap();
                        if select_tables.contains(&table) {
                            used.push(select_file.to_string());
                        }
                    }

                    let statement_type_str = format!("{:?}", statement_type);
                    out.add_row(vec![
                        file.to_string(),
                        statement_type_str,
                        table.to_string(),
                        used.to_owned().join(", "),
                    ]);
                }
            }
        }
    }

    info!("\n{}", out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_prefix_from() {
        let file_name = "src/test/data/insert.sql";
        let file_name_trimmed = trim_prefix_from(&file_name);
        assert_eq!(file_name_trimmed, "insert.sql");
    }

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
    fn test_populate_statements_and_tables_map() {
        let mut statements_map = init_statements_map();
        let mut table_map: HashMap<String, Vec<String>> = HashMap::new();

        populate_maps_for("src/test/data/", &mut statements_map, &mut table_map);

        assert_eq!(statements_map.len(), 8);
        assert_eq!(statements_map.get(&SqlStatement::Select).unwrap().len(), 3);
        assert_eq!(statements_map.get(&SqlStatement::Insert).unwrap().len(), 1);

        assert_eq!(table_map.len(), 4);
        assert_eq!(
            *table_map.get("select_with_cols.sql").unwrap(),
            vec!["video_games".to_string()]
        );
        assert_eq!(
            *table_map.get("insert_vg.sql").unwrap(),
            vec!["video_games".to_string()]
        );
        assert_eq!(
            *table_map.get("select_another.sql").unwrap(),
            vec!["publishers".to_string()]
        );
    }
}

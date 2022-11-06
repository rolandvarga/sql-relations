// use comfy_table::Table; TODO for printing our tables, cols etc.
use std::fs;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static PKG_NAME: &str = env!("CARGO_PKG_NAME");

static LEXER_SEPARATOR: [char; 5] = [' ', ',', '\n', '\t', ';'];
static LEXER_SKIP: [&'static str; 5] = ["", ",", ";", "\n", "\t"];

#[derive(Debug, PartialEq)]
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

fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    info!("running '{}' with version '{}'", PKG_NAME, VERSION);

    let tokens = lex_file("src/test/data/select_with_cols.sql");

    debug!("{:#?}", tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_with_cols() {
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
}

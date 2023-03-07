#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum SyntaxErrors {
    SYNTAX_EXPECTED_NAME,
    SYNTAX_EXPECTED_TOKEN,
    SYNTAX_UNEXPECTED_TOKEN,
    SYNTAX_EXPECTED_TYPE,
    SYNTAX_MISSING_TYPE,
    SYNTAX_EXPECTED_EXPRS,
    SYNTAX_EXPECTED_VALUE,
}

impl SyntaxErrors {
    pub fn code(&self) -> &str {
        match self {
            SyntaxErrors::SYNTAX_EXPECTED_NAME => "E0100",
            SyntaxErrors::SYNTAX_EXPECTED_TOKEN => "E0101",
            SyntaxErrors::SYNTAX_EXPECTED_TYPE => "E0102",
            SyntaxErrors::SYNTAX_UNEXPECTED_TOKEN => "E0103",
            SyntaxErrors::SYNTAX_MISSING_TYPE => "E0104",
            SyntaxErrors::SYNTAX_EXPECTED_EXPRS => "E0105",
            SyntaxErrors::SYNTAX_EXPECTED_VALUE => "E0106",
        }
    }
}

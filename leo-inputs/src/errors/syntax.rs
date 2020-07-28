use crate::ast::Rule;

use pest::error::Error;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("aborting due to syntax error")]
    Error,
}

impl From<Error<Rule>> for SyntaxError {
    fn from(mut error: Error<Rule>) -> Self {
        error = error.renamed_rules(|rule| match *rule {
            Rule::LINE_END => "`;`".to_owned(),
            Rule::type_integer => "`u32`".to_owned(),
            Rule::type_field => "`field`".to_owned(),
            Rule::type_group => "`group`".to_owned(),
            Rule::file => "a table or section".to_owned(),
            Rule::identifier => "a variable name".to_owned(),
            Rule::type_ => "a type".to_owned(),

            rule => format!("{:?}", rule),
        });

        println!("{}\n", error);

        SyntaxError::Error
    }
}

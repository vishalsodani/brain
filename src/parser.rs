use std::str;

use nom::simple_errors::Err;

const COMMENT_START: char = '#';
const STRING_BOUNDARY: char = '"';
const ASSIGNMENT_OPERATOR: char = '=';

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Comment(String),
    Assignment(Option<String>, Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Text(String),
}

pub fn parse(input: &str) -> Result<Vec<Statement>, Err> {
    parse_all_statements(input.as_bytes()).to_result()
}

named!(parse_all_statements< Vec<Statement> >, fold_many0!(statement, Vec::new(), |mut acc: Vec<_>, item| {
    acc.push(item);
    acc
}));

named!(statement<Statement>, ws!(alt!(
    comment => {|content: &str| Statement::Comment(content.to_owned())} |
    assignment => {|(name, expr): (Option<&str>, Expression)| Statement::Assignment(name.map(|s| s.to_owned()), expr)}
)));

named!(comment<&str>,
    do_parse!(
        char!(COMMENT_START) >>
        content: take_until_and_consume!("\n") >>
        (str::from_utf8(content).unwrap())
    )
);

named!(assignment<(Option<&str>, Expression)>,
    do_parse!(
        name: assignment_name >>
        char!(ASSIGNMENT_OPERATOR) >>
        expr: expression >>
        char!('\n') >>
        (name, expr)
    )
);

named!(assignment_name< Option<&str> >,
    map!(
        opt!(assignment_name_impl),
        |n: Option<&'a [u8]>| n.and_then(|s| str::from_utf8(s).ok())
    )
);

// Without this separation, type inference fails
named!(assignment_name_impl<&[u8]>,
    take_while1_s!(is_valid_assignment_name_char)
);

named!(expression<Expression>,
    alt!(
        expr_text => {|text: &str| Expression::Text(text.to_owned())}
    )
);

named!(expr_text<&str>,
    map_res!(
        delimited!(
            char!(STRING_BOUNDARY),
            take_till!(is_string_boundary),
            char!(STRING_BOUNDARY)
        ),
        |s: &'a [u8]| str::from_utf8(s)
    )
);

fn is_valid_assignment_name_char(c: u8) -> bool {
    let ch = c as char;
    ch.is_alphanumeric() || ch == '_'
}

fn is_string_boundary(c: u8) -> bool {
    c as char == STRING_BOUNDARY
}
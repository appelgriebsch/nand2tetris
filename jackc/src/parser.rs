use nom::{IResult, branch::alt, bytes::complete::tag, character::{
        complete::{digit0, multispace0, one_of},
    }, combinator::{all_consuming, map, opt, recognize, value}, error::ParseError, multi::{many0, many1}, sequence::pair};
use nom::{
    bytes::complete::{is_not, take_until},
    character::complete::{alpha1, alphanumeric1},
    sequence::{delimited, tuple},
};
use std::{
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
pub(crate) struct Parser<'a> {
    input_file: &'a str,
    file_content: String,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) enum JackToken<'a> {
    Comment(&'a str),
    Keyword(&'a str),
    Symbol(&'a str),
    IntegerContant(i16),
    StringConstant(&'a str),
    Identifier(&'a str),
}

impl<'a> Parser<'a> {
    pub fn new(input_file: &'a str) -> Self {
        Parser {
            input_file: input_file,
            file_content: String::new(),
        }
    }

    pub fn get_tokens(&'a mut self) -> Result<Vec<JackToken<'a>>, io::Error> {
        File::open(self.input_file)?.read_to_string(&mut self.file_content)?;
        match all_consuming(parse_jackfile)(&self.file_content) {
            Ok((_, result)) => Ok(result),
            Err(e) => {
                eprintln!("{:?}", e);
                panic!();
            }
        }
    }
}

fn parse_jackfile(input: &str) -> IResult<&str, Vec<JackToken>> {
    many1(alt((
        ws(parse_comments),
        ws(parse_keyword),
        ws(parse_symbol),
        ws(parse_identifier),
        ws(parse_integer_constant),
        ws(parse_string_literal)
    )))(input)
}

fn parse_keyword(input: &str) -> IResult<&str, JackToken> {
    let parser = alt((
        tag("class"),
        tag("constructor"),
        tag("function"),
        tag("method"),
        tag("field"),
        tag("static"),
        tag("var"),
        tag("int"),
        tag("char"),
        tag("boolean"),
        tag("void"),
        tag("true"),
        tag("false"),
        tag("null"),
        tag("this"),
        tag("let"),
        tag("do"),
        tag("if"),
        tag("else"),
        tag("while"),
        tag("return"),
    ));
    map(parser, |s| JackToken::Keyword(s))(input)
}

fn parse_symbol(input: &str) -> IResult<&str, JackToken> {
    let parser = alt((
        tag("{"),
        tag("}"),
        tag("("),
        tag(")"),
        tag("["),
        tag("]"),
        tag("."),
        tag(","),
        tag(";"),
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/"),
        value("&amp;", tag("&")),
        tag("|"),
        value("&lt;", tag("<")),
        value("&gt;", tag(">")),
        tag("="),
        tag("~"),
    ));
    map(parser, |s| JackToken::Symbol(s))(input)
}

fn parse_integer_constant(input: &str) -> IResult<&str, JackToken> {
    let parser = recognize(pair(opt(tag("-")), parse_uint));
    map(parser, |s| {
        let n = s.parse::<i16>().unwrap();
        JackToken::IntegerContant(n)
    })(input)
}

fn parse_string_literal(input: &str) -> IResult<&str, JackToken> {
    let parser = delimited(
        tag("\""),
        recognize(take_until("\"")),
        tag("\""),
    );
    map(parser, |s| JackToken::StringConstant(s))(input)
}

fn parse_identifier(input: &str) -> IResult<&str, JackToken> {
    let parser = recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ));
    map(parser, |s| JackToken::Identifier(s))(input)
}

fn parse_comments(input: &str) -> IResult<&str, JackToken> {
    let parser = recognize(
        alt((pinline_comment, peol_comment))
    );
    map(parser, |s| JackToken::Comment(s))(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: Fn(&'a str) -> IResult<&'a str, O, E> {
    delimited(multispace0, inner, multispace0)
}
/// C style comments
fn peol_comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &str, E> {
    let parser = pair(tag("//"), is_not("\n\r"));
    map(parser, |s| s.1)(input)
}
fn pinline_comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &str, E> {
    let parser = tuple((tag("/*"), take_until("*/"), tag("*/")));
    map(parser, |s| s.1)(input)
}

// integer constants
fn take_digit1to9(input: &str) -> IResult<&str, char> {
    one_of("123456789")(input)
}
fn parse_uint(input: &str) -> IResult<&str, &str> {
    alt((tag("0"), recognize(pair(take_digit1to9, digit0))))(input)
}
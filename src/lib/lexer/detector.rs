use anyhow::Result;
use nom::{
	branch::alt,
	bytes::complete::{tag, take_until},
	character::complete::{alpha1, alphanumeric1, anychar, char, digit1, line_ending, space1, tab},
	combinator::{map, map_res, recognize, value},
	multi::{many0, many1, many_m_n},
	sequence::{delimited, terminated, tuple},
	IResult,
};

use super::tokens::token_type::{Keyword, Literal, Operator, Punctuation, TokenType};

pub fn detect_punctuation(input: &[u8]) -> IResult<&[u8], Punctuation> {
	let (tail, keyword) = alt((
		value(Punctuation::BracketOpen, tag(b"(")),
		value(Punctuation::BracketClose, tag(b")")),
		value(Punctuation::BracketCurlyOpen, tag(b"{")),
		value(Punctuation::BracketCurlyClose, tag(b"}")),
		value(Punctuation::QuoteSingle, tag(b"'")),
		value(Punctuation::QuoteDouble, tag(b"\"")),
		value(Punctuation::Semicolon, tag(b";")),
		value(Punctuation::Colon, tag(b":")),
		value(Punctuation::Pipe, tag(b"|")),
		value(Punctuation::Ampersand, tag(b"&")),
		value(Punctuation::Dot, tag(b".")),
		value(Punctuation::Comma, tag(b",")),
		value(Punctuation::Space, space1),
		value(Punctuation::Tab, tab),
		value(Punctuation::Eol, line_ending),
	))(input)?;
	Ok((tail, keyword))
}

pub fn detect_operator(input: &[u8]) -> IResult<&[u8], Operator> {
	let (tail, op) = alt((
		value(Operator::Not, tag(b"!")),
		value(Operator::NotEq, tag(b"!=")),
		value(Operator::Eq, tag(b"=")),
		value(Operator::EqEq, tag(b"==")),
		value(Operator::Gt, tag(b">")),
		value(Operator::Gte, tag(b">=")),
		value(Operator::Lt, tag(b"<")),
		value(Operator::Lte, tag(b"<=")),
		value(Operator::Add, tag(b"+")),
		value(Operator::Sub, tag(b"-")),
		value(Operator::Mul, tag(b"*")),
		value(Operator::Div, tag(b"/")),
	))(input)?;
	Ok((tail, op))
}

pub fn detect_keyword(input: &[u8]) -> IResult<&[u8], Keyword> {
	let (tail, kw) = alt((
		value(Keyword::And, tag(b"and")),
		value(Keyword::Or, tag(b"or")),
		value(Keyword::Class, tag(b"class")),
		value(Keyword::If, tag(b"if")),
		value(Keyword::Else, tag(b"else")),
		value(Keyword::True, tag(b"true")),
		value(Keyword::False, tag(b"false")),
		value(Keyword::Function, tag(b"fn")),
		value(Keyword::For, tag(b"for")),
		value(Keyword::While, tag(b"while")),
		value(Keyword::Nil, tag(b"nil")),
		value(Keyword::Print, tag(b"print")),
		value(Keyword::Return, tag(b"return")),
		value(Keyword::Super, tag(b"super")),
		value(Keyword::This, tag(b"this")),
		value(Keyword::Var, tag(b"var")),
	))(input)?;
	Ok((tail, kw))
}

pub fn decimal(input: &[u8]) -> IResult<&[u8], Literal> {
	let (tail, token): _ = map_res(
		recognize(tuple((
			many_m_n(0, 1, char('-')),
			many1(terminated(digit1, many0(char('_')))),
			many_m_n(
				0,
				1,
				tuple((char('.'), many1(terminated(digit1, many0(char('_')))))),
			),
		))),
		|token: &[u8]| -> Result<f64> {
			let n_str = token
				.iter()
				.copied()
				.filter(|c| *c != b'_')
				.collect::<Vec<_>>();
			let n_str = std::str::from_utf8(&n_str[..])?;
			let n = n_str.parse::<f64>()?;
			Ok(n)
		},
	)(input)?;

	Ok((tail, Literal::Number(token)))
}

pub fn string(input: &[u8]) -> IResult<&[u8], Literal> {
	let (tail, token) = alt((
		delimited(char('\''), take_until("'"), char('\'')),
		delimited(char('"'), take_until("\""), char('"')),
	))(input)?;
	Ok((tail, Literal::String(token)))
}

pub fn identifier(input: &[u8]) -> IResult<&[u8], Literal> {
	let (tail, token) = recognize(tuple((
		many1(alt((alpha1, tag("_"), tag("$")))),
		many0(alt((alphanumeric1, tag("_"), tag("$")))),
	)))(input)?;
	Ok((tail, Literal::Identifier(token)))
}

pub fn detect(input: &[u8]) -> IResult<&[u8], TokenType> {
	let (tail, token) = alt((
		map(detect_keyword, TokenType::Keyword),
		map(detect_operator, TokenType::Operator),
		map(detect_punctuation, TokenType::Punctuation),
		map(alt((decimal, string, identifier)), TokenType::Literal),
		map(many1(anychar), |t| {
			let t: String = t.iter().collect();
			TokenType::Generic(t)
		}),
	))(input)?;
	Ok((tail, token))
}

use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{line_ending, space1, tab},
	combinator::{map, value},
	IResult,
};

use super::tokens::token_type::{Keyword, Operator, Punctuation, TokenType};

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

pub fn detect(input: &[u8]) -> IResult<&[u8], TokenType> {
	let (tail, token) = alt((
		map(detect_keyword, TokenType::Keyword),
		map(detect_operator, TokenType::Operator),
		map(detect_punctuation, TokenType::Punctuation),
	))(input)?;
	Ok((tail, token))
}

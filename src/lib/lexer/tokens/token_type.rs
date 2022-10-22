#[derive(Debug, Clone)]
pub enum TokenType<'t> {
	Punctuation(Punctuation),
	Operator(Operator),
	Keyword(Keyword),
	Literal(Literal<'t>),
	Generic(String),
}

#[derive(Debug, Clone)]
pub enum Punctuation {
	BracketOpen,
	BracketClose,
	BracketCurlyOpen,
	BracketCurlyClose,
	QuoteSingle,
	QuoteDouble,
	Semicolon,
	Colon,
	Pipe,
	Ampersand,
	Dot,
	Comma,
	Space,
	Tab,
	/// End of line
	Eol,
	/// End of file
	Eof,
}

#[derive(Debug, Clone)]
pub enum Operator {
	/// !
	Not,
	/// !=
	NotEq,
	/// =
	Eq,
	/// ==
	EqEq,
	/// >
	Gt,
	/// >=
	Gte,
	/// <
	Lt,
	/// <=
	Lte,
	/// +
	Add,
	/// -
	Sub,
	/// *
	Mul,
	/// /
	Div,
}
impl Operator {
	pub fn to_str(&self) -> &str {
		match self {
			Self::Not => "!",
			Self::NotEq => "!=",
			Self::Eq => "=",
			Self::EqEq => "==",
			Self::Gt => ">",
			Self::Gte => ">=",
			Self::Lt => "<",
			Self::Lte => "<=",
			Self::Add => "+",
			Self::Sub => "-",
			Self::Mul => "*",
			Self::Div => "/",
		}
	}
}

#[derive(Debug, Clone)]
pub enum Keyword {
	And,
	Or,
	Class,
	If,
	Else,
	True,
	False,
	Function,
	For,
	While,
	Nil,
	Print,
	Return,
	Super,
	This,
	Var,
}

#[derive(Debug, Clone)]
pub enum Literal<'l> {
	String(&'l [u8]),
	Number(f64),
	Identifier(&'l [u8]),
}

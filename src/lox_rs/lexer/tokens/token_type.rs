#[derive(Debug, Clone, PartialEq)]
pub enum TokenType<'t> {
	Punctuation(Punctuation),
	Operator(Operator),
	Keyword(Keyword),
	Literal(Literal<'t>),
	Identifier(&'t [u8]),
	Generic(Vec<char>),
	EndOfLine,
	EndOfFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Punctuation {
	/// `(`
	BracketOpen,
	/// `)`
	BracketClose,
	/// `{`
	BracketCurlyOpen,
	/// `}`
	BracketCurlyClose,
	/// `'`
	QuoteSingle,
	/// `"`
	QuoteDouble,
	/// `;`
	Semicolon,
	/// `:`
	Colon,
	/// `|`
	Pipe,
	/// `&`
	Ampersand,
	/// `.`
	Dot,
	/// `,`
	Comma,
	/// ` `
	Space,
	/// `\t`
	Tab,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
	/// &&
	And,
	/// ||
	Or,
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
			Operator::And => "&&",
			Operator::Or => "||",
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
	Class,
	If,
	Else,
	Function,
	For,
	While,
	Print,
	Return,
	Super,
	This,
	Var,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'l> {
	String(&'l [u8]),
	Number(f64),
	Boolean(bool),
	Null,
}

use crate::types::{FloatType, IntType, Source};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TokenType {
    And,
    Break,
    Do,
    Else,
    ElseIf,
    End,
    False,
    For,
    Function,
    Goto,
    // '//'
    IDiv,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,
    // ..
    Concat,
    // ...
    Dots,
    // ==
    Eq,
    // >=
    Ge,
    // <=
    Le,
    // ~=
    Ne,
    // <<
    Shl,
    // >>
    Shr,
    // ::
    DbColon,
    Eos,
    // float number
    Flt,
    // int number
    Int,
    // name
    Name,
    // string literal
    String,
    // () [] {}
    Lp,
    Rp,
    Ls,
    Rs,
    Lb,
    Rb,
    // + - * / % ^ #
    Add,
    Minus,
    Mul,
    Div,
    Mod,
    Pow,
    Len,
    // =
    Assign,
    // < >
    Lt,
    Gt,
    // & | ~
    BAnd,
    BOr,
    BXor,
    // : , ;
    Colon,
    Comma,
    Semi,
    // .
    Attr,
    // single line coment
    SComment,
    // multi-line comment
    MComment,
}

impl TokenType {
    // convert keyword to token type.
    pub fn from_keyword(word: &str) -> Option<TokenType> {
        match word {
            "and" => Some(TokenType::And),
            "break" => Some(TokenType::Break),
            "do" => Some(TokenType::Do),
            "else" => Some(TokenType::Else),
            "elseif" => Some(TokenType::ElseIf),
            "end" => Some(TokenType::End),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "function" => Some(TokenType::Function),
            "goto" => Some(TokenType::Goto),
            "if" => Some(TokenType::If),
            "in" => Some(TokenType::In),
            "local" => Some(TokenType::Local),
            "nil" => Some(TokenType::Nil),
            "not" => Some(TokenType::Not),
            "or" => Some(TokenType::Or),
            "repeat" => Some(TokenType::Repeat),
            "return" => Some(TokenType::Return),
            "then" => Some(TokenType::Then),
            "true" => Some(TokenType::True),
            "until" => Some(TokenType::Until),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }

    pub fn to_keyword(t: TokenType) -> Option<String> {
        match t {
            TokenType::And => Some(String::from("and")),
            TokenType::Break => Some(String::from("break")),
            TokenType::Do => Some(String::from("do")),
            TokenType::Else => Some(String::from("else")),
            TokenType::ElseIf => Some(String::from("elseif")),
            TokenType::End => Some(String::from("end")),
            TokenType::False => Some(String::from("false")),
            TokenType::For => Some(String::from("for")),
            TokenType::Function => Some(String::from("function")),
            TokenType::Goto => Some(String::from("goto")),
            TokenType::If => Some(String::from("if")),
            TokenType::In => Some(String::from("in")),
            TokenType::Local => Some(String::from("local")),
            TokenType::Nil => Some(String::from("nil")),
            TokenType::Not => Some(String::from("not")),
            TokenType::Or => Some(String::from("or")),
            TokenType::Repeat => Some(String::from("repeat")),
            TokenType::Return => Some(String::from("return")),
            TokenType::Then => Some(String::from("then")),
            TokenType::True => Some(String::from("true")),
            TokenType::Until => Some(String::from("until")),
            TokenType::While => Some(String::from("while")),
            _ => None,
        }
    }
}

impl From<u8> for TokenType {
    fn from(n: u8) -> TokenType {
        match n {
            0 => TokenType::And,
            1 => TokenType::Break,
            2 => TokenType::Do,
            3 => TokenType::Else,
            4 => TokenType::ElseIf,
            5 => TokenType::End,
            6 => TokenType::False,
            7 => TokenType::For,
            8 => TokenType::Function,
            9 => TokenType::Goto,
            10 => TokenType::IDiv,
            11 => TokenType::If,
            12 => TokenType::In,
            13 => TokenType::Local,
            14 => TokenType::Nil,
            15 => TokenType::Not,
            16 => TokenType::Or,
            17 => TokenType::Repeat,
            18 => TokenType::Return,
            19 => TokenType::Then,
            20 => TokenType::True,
            21 => TokenType::Until,
            22 => TokenType::While,
            23 => TokenType::Concat,
            24 => TokenType::Dots,
            25 => TokenType::Eq,
            26 => TokenType::Ge,
            27 => TokenType::Le,
            28 => TokenType::Ne,
            29 => TokenType::Shl,
            30 => TokenType::Shr,
            31 => TokenType::DbColon,
            32 => TokenType::Eos,
            33 => TokenType::Flt,
            34 => TokenType::Int,
            35 => TokenType::Name,
            36 => TokenType::String,
            37 => TokenType::Lp,
            38 => TokenType::Rp,
            39 => TokenType::Ls,
            40 => TokenType::Rs,
            41 => TokenType::Lb,
            42 => TokenType::Rb,
            43 => TokenType::Add,
            44 => TokenType::Minus,
            45 => TokenType::Mul,
            46 => TokenType::Div,
            47 => TokenType::Mod,
            48 => TokenType::Pow,
            49 => TokenType::Len,
            50 => TokenType::Assign,
            51 => TokenType::Lt,
            52 => TokenType::Gt,
            53 => TokenType::BAnd,
            54 => TokenType::BOr,
            56 => TokenType::BXor,
            57 => TokenType::Colon,
            58 => TokenType::Comma,
            59 => TokenType::Semi,
            60 => TokenType::Attr,
            61 => TokenType::SComment,
            62 => TokenType::MComment,
            _ => TokenType::Add,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    None,
    Float(FloatType),
    Int(IntType),
    Str(String),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub t: TokenType,
    pub value: TokenValue,
    pub source: Source,
}

impl Token {
    pub fn get_float(&self) -> FloatType {
        match self.value {
            TokenValue::Float(f) => f,
            _ => unreachable!(),
        }
    }
    pub fn get_int(&self) -> IntType {
        match self.value {
            TokenValue::Int(i) => i,
            _ => unreachable!(),
        }
    }
    pub fn get_string(&self) -> String {
        match &self.value {
            TokenValue::Str(s) => s.clone(),
            _ => unreachable!(),
        }
    }
    pub fn is_comment(&self) -> bool {
        self.t == TokenType::SComment || self.t == TokenType::MComment
    }
}

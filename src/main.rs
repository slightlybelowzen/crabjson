#[derive(Debug)]
enum TokenType {
    SemiColon,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    Identifier(String),
    EOF,
}


#[derive(Debug)]
// FIXME: this shoudn't be required
#[allow(dead_code)]
struct ParseError(String);

impl ParseError {
    fn new(msg: String) -> Self {
        ParseError(msg)
    }
}

#[derive(Debug)]
// FIXME: this shoudn't be required
#[allow(dead_code)]
struct LexError(String);

impl LexError {
    fn new(msg: String) -> Self {
        LexError(msg)
    }
}

#[derive(Debug, PartialEq)]
enum RootNodeType {
    ObjectNode,
    #[allow(dead_code)]
    ArrayNode
}

#[derive(Debug)]
struct RootNode {
    #[allow(dead_code)]
    root: RootNodeType,
}

impl RootNode {}

// #[derive(Debug)]
// struct ArrayNode {
//     body: Vec<String>,
// }

// #[derive(Debug)]
// struct ObjectNode {
//     body: Vec<Option<KeyValueNode>>,
// }

// #[derive(Debug)]
// struct KeyValueNode {
//     key: String,
//     value: String,
// }

fn tokenize(input: &str) -> Result<Vec<TokenType>, LexError> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            ch if ch.is_alphabetic() => {
                let mut identifier = String::new();
                identifier.push(ch);
                while let Some(ch) = iter.peek() {
                    if ch.is_alphabetic() {
                        identifier.push(ch.clone());
                        iter.next();
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "true" => {tokens.push(TokenType::Bool(true))},
                    "false" => {tokens.push(TokenType::Bool(false))},
                    "null" => {tokens.push(TokenType::Null)},
                    _ => {tokens.push(TokenType::Identifier(identifier))}
                }
            },
            ch if ch.is_digit(10) => {
                let mut num = String::new();
                num.push(ch);
                while let Some(ch) = iter.peek() {
                    if ch.is_digit(10) || ch.clone() == '.' {
                        num.push(ch.clone());
                        iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push(TokenType::Number(num.parse::<f64>().unwrap()));
            },
            '{' => tokens.push(TokenType::LeftBrace),
            '}' => tokens.push(TokenType::RightBrace),
            '[' => tokens.push(TokenType::LeftBracket),
            ']' => tokens.push(TokenType::RightBracket),
            ',' => tokens.push(TokenType::Comma),
            ':' => tokens.push(TokenType::Colon),
            ';' => tokens.push(TokenType::SemiColon),
            _ => return Err(LexError::new(format!("unexpected character: {:?}", ch).to_string()))
        }
    }
    tokens.push(TokenType::EOF);
    return Ok(tokens);
}

fn parse(tokens: &Vec<TokenType>) -> Result<RootNode, ParseError> {
    let mut iter = tokens.iter().peekable();
    let mut ast = RootNode { root: RootNodeType::ObjectNode };
    while let Some(token) = iter.next() {
        match token {
            TokenType::LeftBrace => {continue;},
            TokenType::LeftBracket => {
              ast = RootNode { root: RootNodeType::ArrayNode };
            },
            _ => return Err(ParseError::new({format!("unexpected token {:?}", token)}.to_string()))
        }
    }

    Ok(ast)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: crabjson [file ...]");
    }
    let files = &args[1..];
    for file in files {
        let contents = std::fs::read_to_string(file).unwrap();
        let tokens = tokenize(&contents).unwrap();
        println!("tokens for {}\n  {:?}", &file, tokens);
        let ast = parse(&tokens);
        println!("ast for {}\n  {:?}", &file, ast);
    }
}

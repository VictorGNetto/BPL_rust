use super::core::*;
use super::*;

#[derive(Debug)]
pub struct Const {
    pub n: i32,
}

#[derive(Debug)]
pub struct Parameter {
    pub id: Identifier,
    pub base_type: BaseType,
    pub address: u32,
    pub register: String,
}

#[derive(Debug)]
pub struct Variable {
    pub id: Identifier,
    pub base_type: BaseType,
    pub address: u32,
}

#[derive(Debug)]
pub enum Token {
    ConstTk(Const),
    ParameterTk(Parameter),
    VariableTk(Variable),
}

fn is_token(tk: &str) -> bool {
    re::int_const().is_match(tk) ||
    re::int_variable().is_match(tk) ||
    re::int_parameter().is_match(tk) ||
    re::array_variable().is_match(tk) ||
    re::array_parameter().is_match(tk)
}

fn str_to_token(token_str: &str, context: &Context) -> Token {
    if re::int_const().is_match(token_str) {
        let caps = re::int_const().captures(token_str).unwrap();
        let n: i32 = caps["n"].parse().unwrap();
        Token::ConstTk(
            Const {
                n,
            }
        )
    } else if re::int_parameter().is_match(token_str) {
        let caps = re::int_parameter().captures(token_str).unwrap();
        let id: u8 = caps["pid"].parse().unwrap();

        let id = Identifier::Parameter(id);
        let base_type = BaseType::Int;
        let address: u32;
        let register;
        match id {
            Identifier::Parameter(1) => {
                address = 8;
                register = String::from("%edi");
            }
            Identifier::Parameter(2) => {
                address = 16;
                register = String::from("%esi");
            }
            Identifier::Parameter(3) => {
                address = 24;
                register = String::from("%edx");
            }
            _ => {
                address = 0;
                register = String::from("");
            }
        }

        Token::ParameterTk(
            Parameter {
                id,
                base_type,
                address,
                register,
            }
        )
    } else if re::int_variable().is_match(token_str) {
        let caps = re::int_variable().captures(token_str).unwrap();
        let id: u8 = caps["vid"].parse().unwrap();
        let address = context.local_variables[id as usize - 1].address;

        let id = Identifier::Variable(id);
        let base_type = BaseType::Int;
        Token::VariableTk(
            Variable {
                id,
                base_type,
                address,
            }
        )
    } else if re::array_variable().is_match(token_str) {
        let caps = re::array_variable().captures(token_str).unwrap();
        let id: u8 = caps["vid"].parse().unwrap();
        let address = context.local_variables[id as usize - 1].address;

        let id = Identifier::Variable(id);
        let base_type = BaseType::Array;
        Token::VariableTk(
            Variable {
                id,
                base_type,
                address,
            }
        )
    } else { // re::array_parameter().is_match(token_str)
        let caps = re::array_parameter().captures(token_str).unwrap();
        let id: u8 = caps["pid"].parse().unwrap();

        let id = Identifier::Parameter(id);
        let base_type = BaseType::Array;
        let address: u32;
        let register;
        match id {
            Identifier::Parameter(1) => {
                address = 8;
                register = String::from("%rdi");
            }
            Identifier::Parameter(2) => {
                address = 16;
                register = String::from("%rsi");
            }
            Identifier::Parameter(3) => {
                address = 24;
                register = String::from("%rdx");
            }
            _ => {
                address = 0;
                register = String::from("");
            }
        }

        Token::ParameterTk(
            Parameter {
                id,
                base_type,
                address,
                register,
            }
        )
    }
}

pub fn parse(line: &str, context: &Context) -> Vec<Token> {
    let tokens_str: Vec<&str> = line.split_whitespace().filter(
        |&token| is_token(token)
    ).collect();
    let mut tokens = Vec::new();
    for token_str in tokens_str {
        tokens.push(str_to_token(token_str, context));
    }

    tokens
}
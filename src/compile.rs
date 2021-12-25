use super::core::*;
use super::token;
use super::re;

pub fn assignment(line: &String, context: &Context) {
    println!("\n    # {}", line);

    let tokens = token::parse(line.as_str(), &context);

    if re::expression_assignment().is_match(line.as_str()) {
        let caps = re::expression_assignment().captures(line.as_str()).unwrap();
        let op: char = caps["op"].parse().unwrap();
        compile_expression_assignment(&tokens, op);
    } else if re::function_return_assignment().is_match(line.as_str()) {
        let caps = re::function_return_assignment().captures(line.as_str()).unwrap();
        let fid: u32 = caps["fid"].parse().unwrap();
        compile_function_return_assignment(&tokens, &context, Identifier::Function(fid));
    } else { // re::simple_assignment().is_match(line.as_str())
        compile_simple_assignment(&tokens);
    }
}

fn compile_expression_assignment(tokens: &Vec<token::Token>, op: char) {
    let dst_address = match &tokens[0] {
        token::Token::VariableTk(v) => v.address,
        _ => 0,
    };

    match &tokens[1] {
        token::Token::ConstTk(c) => {
            println!("    movl ${}, %eax", c.n);
        }
        token::Token::ParameterTk(p) => {
            println!("    movl {}, %eax", p.register);
        }
        token::Token::VariableTk(v) => {
            println!("    movl -{}(%rbp), %eax", v.address);
        }
    }

    match &tokens[2] {
        token::Token::ConstTk(c) => {
            println!("    movl ${}, %ecx", c.n);
        }
        token::Token::ParameterTk(p) => {
            println!("    movl {}, %ecx", p.register);
        }
        token::Token::VariableTk(v) => {
            println!("    movl -{}(%rbp), %ecx", v.address);
        }
    }

    match op {
        '+' => {
            println!("    addl %ecx, %eax");
        }
        '-' => {
            println!("    subl %ecx, %eax");
        }
        '*' => {
            println!("    imull %ecx, %eax");
        }
        '/' => {
            println!("    movq %rdx, %r8");
            println!("    cltd");
            println!("    idivl %ecx");
            println!("    movq %r8, %rdx");
        }
        _ => (),
    }

    println!("    movl %eax, -{}(%rbp)", dst_address);
}

fn compile_function_return_assignment(tokens: &Vec<token::Token>, context: &Context, identifier: Identifier) {
    let fid = match identifier {
        Identifier::Function(n) => n,
        _ => 0,
    };

    let dst_address = match &tokens[0] {
        token::Token::VariableTk(v) => v.address,
        _ => 0,
    };

    match context.parameters_count {
        1 => {
            println!("    movq %rdi, -8(%rbp)");
        }
        2 => {
            println!("    movq %rdi, -8(%rbp)");
            println!("    movq %rsi, -16(%rbp)");
        }
        3 => {
            println!("    movq %rdi, -8(%rbp)");
            println!("    movq %rsi, -16(%rbp)");
            println!("    movq %rdx, -24(%rbp)");
        }
        _ => (),
    }

    match tokens.len() {
        2 => {
            setup_parameter(1, &tokens[1]);
        }
        3 => {
            setup_parameter(1, &tokens[1]);
            setup_parameter(2, &tokens[2]);
        }
        4 => {
            setup_parameter(1, &tokens[1]);
            setup_parameter(2, &tokens[2]);
            setup_parameter(3, &tokens[3]);
        }
        _ => (),
    }

    println!("    call f{}", fid);
    println!("    movl %eax, -{}(%rbp)", dst_address);

    match context.parameters_count {
        1 => {
            println!("    movq -8(%rbp), %rdi");
        }
        2 => {
            println!("    movq -8(%rbp), %rdi");
            println!("    movq -16(%rbp), %rsi");
        }
        3 => {
            println!("    movq -8(%rbp), %rdi");
            println!("    movq -16(%rbp), %rsi");
            println!("    movq -24(%rbp), %rdx");
        }
        _ => (),
    }
}

fn setup_parameter(n: u32, token: &token::Token) {
    let posfix;
    match n {
        1 => posfix = "di",
        2 => posfix = "si",
        3 => posfix = "dx",
        _ => posfix = "",
    } 

    match token {
        token::Token::ConstTk(c) => {
            println!("    movl ${}, %e{} # {}o parameter", c.n, posfix, n);
        }
        token::Token::ParameterTk(p) => {
            match p.base_type {
                BaseType::Array => {
                    println!("    movq -{}(%rbp), %r{} # {}o parameter", p.address, posfix, n);
                }
                BaseType::Int => {
                    println!("    movq -{}(%rbp), %rax", p.address);
                    println!("    movl %eax, %e{} # {}o parameter", posfix, n);
                }
            }
        }
        token::Token::VariableTk(v) => {
            match v.base_type {
                BaseType::Array => {
                    println!("    movq -{}(%rbp), %r{} # {}o parameter", v.address, posfix, n);
                }
                BaseType::Int => {
                    println!("    movl -{}(%rbp), %e{} # {}o parameter", v.address, posfix, n);
                }
            }
        }
    }
}

fn compile_simple_assignment(tokens: &Vec<token::Token>) {
    let dst_address = match &tokens[0] {
        token::Token::VariableTk(v) => v.address,
        _ => 0,
    };
    let src = &tokens[1];

    match src {
        token::Token::ConstTk(c) => {
            println!("    movl ${}, -{}(%rbp)", c.n, dst_address);
        }
        token::Token::ParameterTk(p) => {
            println!("    movl {}, -{}(%rbp)", p.register, dst_address);
        }
        token::Token::VariableTk(v) => {
            println!("    movl -{}(%rbp), %eax", v.address);
            println!("    movl %eax, -{}(%rbp)", dst_address);
        }
    }
}

pub fn get(line: &String, context: &Context) {
    println!("\n    # {}", line);

    let tokens = token::parse(line.as_str(), &context);

    let index = match &tokens[1] {
        token::Token::ConstTk(c) => c.n,
        _ => 0,
    };
    
    match &tokens[0] {
        token::Token::ParameterTk(p) => {
            println!("    movq {}, %r9", p.register);
        }
        token::Token::VariableTk(v) => {
            println!("    movq -{}(%rbp), %r9", v.address);
        }
        _ => (),
    };
    println!("    movq ${}, %r8", index);
    println!("    imulq $4, %r8");
    println!("    addq %r8, %r9");
    match &tokens[2] {
        token::Token::ParameterTk(p) => {
            println!("    movl (%r9), {}", p.register);
        }
        token::Token::VariableTk(v) => {
            println!("    movq (%r9), -{}(%rbp)", v.address);
        }
        _ => (),
    }
}

pub fn set(line: &String, context: &Context) {
    println!("\n    # {}", line);

    let tokens = token::parse(line.as_str(), &context);

    let index = match &tokens[1] {
        token::Token::ConstTk(c) => c.n,
        _ => 0,
    };
    
    match &tokens[0] {
        token::Token::ParameterTk(p) => {
            println!("    movq {}, %r9", p.register);
        }
        token::Token::VariableTk(v) => {
            println!("    movq -{}(%rbp), %r9", v.address);
        }
        _ => (),
    };
    println!("    movq ${}, %r8", index);
    println!("    imulq $4, %r8");
    println!("    addq %r8, %r9");
    match &tokens[2] {
        token::Token::ConstTk(c) => {
            println!("    movl ${}, (%r9)", c.n);
        }
        token::Token::ParameterTk(p) => {
            println!("    movl {}, (%r9)", p.register);
        }
        token::Token::VariableTk(v) => {
            println!("    movl -{}(%rbp), %eax", v.address);
            println!("    movl %eax, (%r9)");
        }
    }
}

pub fn conditional(line: &String, context: &Context, n: u32) {
    println!("\n    # {}", line);
    println!("# begin_if{}:", n);

    let token = &token::parse(line.as_str(), &context)[0];
    match token {
        token::Token::ConstTk(c) => {
            println!("    movl ${}, %eax", c.n);
        }
        token::Token::ParameterTk(p) => {
            println!("    movl {}, %eax", p.register);
        }
        token::Token::VariableTk(v) => {
            println!("    movl -{}(%rbp), %eax", v.address);
        }
    }
    println!("    cmpl $0, %eax");
    println!("    je end_if2");
}

pub fn return_function(line: &String, context: &Context) {
    println!("\n    # {}", line);

    let token = &token::parse(line.as_str(), &context)[0];
    match token {
        token::Token::ConstTk(c) => {
            println!("    movl ${}, %eax", c.n);
        }
        token::Token::ParameterTk(p) => {
            println!("    movl {}, %eax", p.register);
        }
        token::Token::VariableTk(v) => {
            println!("    movl -{}(%rbp), %eax", v.address);
        }
    }
}
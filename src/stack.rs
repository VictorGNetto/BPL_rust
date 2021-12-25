use super::core::*;
use super::*;

pub fn allocate(parameters_count: u8) -> Context {
    let mut address: u32 = (parameters_count as u32) * 8;
    let mut variables_count = 0;
    let mut local_variables: [Variable; 5] = [
        Variable {
            address: 0,
            variable_type: BaseType::Int,
        },
        Variable {
            address: 0,
            variable_type: BaseType::Int,
        },
        Variable {
            address: 0,
            variable_type: BaseType::Int,
        },
        Variable {
            address: 0,
            variable_type: BaseType::Int,
        },
        Variable {
            address: 0,
            variable_type: BaseType::Int,
        },
    ];

    while let Some(line) = read_line() {
        if re::int_variable_def().is_match(line.as_str()) {
            let caps = re::int_variable().captures(line.as_str()).unwrap();
            variables_count = caps["vid"].parse().unwrap();
            address += 4;
            local_variables[variables_count as usize - 1].address = address;
            local_variables[variables_count as usize - 1].variable_type = BaseType::Int;
        } else if re::array_variable_def().is_match(line.as_str()) {
            let caps = re::array_variable().captures(line.as_str()).unwrap();
            variables_count = caps["vid"].parse().unwrap();
            let caps = re::int_const().captures(line.as_str()).unwrap();
            let size: u32 = caps["n"].parse().unwrap();
            address += 4 * size;
            local_variables[variables_count as usize - 1].address = address;
            local_variables[variables_count as usize - 1].variable_type = BaseType::Array;
        } else if re::enddef().is_match(line.as_str()) {
            break;
        }
    }

    let stack_bytes_used: u32;
    if address % 16 == 0 {
        stack_bytes_used = address;
    } else {
        stack_bytes_used = address + 16 - address % 16;
    }

    Context {
        parameters_count,
        variables_count,
        local_variables,
        stack_bytes_used,
    }
}

pub fn show_locations(context: &Context) {
    match context.parameters_count {
        1 => {
            println!("    # when needed %rdi is saved at -8(%rbp) before function call");
        }
        2 => {
            println!("    # when needed %rdi is saved at -8(%rbp) before function call");
            println!("    # when needed %rsi is saved at -16(%rbp) before function call");
        }
        3 => {
            println!("    # when needed %rdi is saved at -8(%rbp) before function call");
            println!("    # when needed %rsi is saved at -16(%rbp) before function call");
            println!("    # when needed %rdx is saved at -24(%rbp) before function call");
        }
        _ => (),
    }

    for i in 0..context.variables_count {
        let address = context.local_variables[i as usize].address;
        match context.local_variables[i as usize].variable_type {
            BaseType::Int => {
                println!("    # vi{} is kept at -{}(%rbp)", i + 1, address);
            }
            BaseType::Array => {
                println!("    # va{} is kept at -{}(%rbp)", i + 1, address);
            }
        }
    }
}

use bpl::{core::*, stack, re, compile};

fn main() {
    println!(".text");

    while let Some(line) = bpl::read_line() {
        if re::function_def().is_match(line.as_str()) {
            let caps = re::function_def().captures(line.as_str()).unwrap();
            let fid: u32 = caps["fid"].parse().unwrap();

            let parameters_count = 
                re::int_parameter().find_iter(line.as_str()).count()
                + re::array_parameter().find_iter(line.as_str()).count();

            println!("\n    # {}", line);
            compile_function(parameters_count as u8, Identifier::Function(fid));
        }
    }
}

fn compile_function(parameters_count: u8, identifier: Identifier) {
    let fid = match identifier {
        Identifier::Function(n) => n,
        _ => 0,
    };

    let context = stack::allocate(parameters_count);

    println!(".globl f{}", fid);
    println!("f{}:", fid);
    println!("    pushq %rbp");
    println!("    movq %rsp, %rbp");
    if context.stack_bytes_used != 0 {
        println!("    subq ${}, %rsp", context.stack_bytes_used);
    }
    println!();
    stack::show_locations(&context);

    let mut inside_conditional = false;
    let mut conditional_count: u32 = fid * 1000;

    while let Some(line) = bpl::read_line() {
        if re::assignment().is_match(line.as_str()) {
            compile::assignment(&line, &context);
        } else if re::array_access_get().is_match(line.as_str()) {
            compile::get(&line, &context);
        } else if re::array_access_set().is_match(line.as_str()) {
            compile::set(&line, &context);
        } else if re::begin_conditional().is_match(line.as_str()) {
            compile::conditional(&line, &context, conditional_count);
            inside_conditional = true;
        } else if re::end_conditional().is_match(line.as_str()) {
            println!("end_if{}:", conditional_count);
            inside_conditional = false;
            conditional_count += 1;

        } else if re::function_return().is_match(line.as_str()) {
            compile::return_function(&line, &context);
            if inside_conditional {
                println!("    jmp return_f{}", fid);
            }
        } else if re::function_end().is_match(line.as_str()) {
            break;
        }
    }

    println!("\nreturn_f{}:", fid);
    println!("    leave");
    println!("    ret");
}
#[derive(Debug)]
pub enum Identifier {
    Function(u32),
    Parameter(u8),
    Variable(u8),
}

#[derive(Debug)]
pub enum BaseType {
    Int,
    Array,
}

pub struct Variable {
    pub address: u32,
    pub variable_type: BaseType,
}

pub struct Context {
    pub parameters_count: u8,
    pub variables_count: u8,
    pub local_variables: [Variable; 5],
    pub stack_bytes_used: u32,
}
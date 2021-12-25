use regex::Regex;

pub fn function_def() -> regex::Regex {
    Regex::new(r"function f(?P<fid>[0-9]*)").unwrap()
}

pub fn function_end() -> regex::Regex {
    Regex::new(r"^end$").unwrap()
}

pub fn enddef() -> regex::Regex {
    Regex::new("^enddef$").unwrap()
}

pub fn assignment() -> regex::Regex {
    Regex::new(r"^vi\d = ").unwrap()
}

pub fn expression_assignment() -> regex::Regex {
    Regex::new(r"(?P<op>[-+*/])").unwrap()
}

pub fn function_return_assignment() -> regex::Regex {
    Regex::new(r"call f(?P<fid>[0-9]*)").unwrap()
}

pub fn array_access_get() -> regex::Regex {
    Regex::new(r"^get").unwrap()
}

pub fn array_access_set() -> regex::Regex {
    Regex::new(r"^set").unwrap()
}

pub fn begin_conditional() -> regex::Regex {
    Regex::new(r"^if").unwrap()
}

pub fn end_conditional() -> regex::Regex {
    Regex::new(r"^endif").unwrap()
}

pub fn function_return() -> regex::Regex {
    Regex::new(r"^return").unwrap()
}

pub fn int_variable() -> regex::Regex {
    Regex::new(r"vi(?P<vid>[1-5])").unwrap()
}

pub fn int_parameter() -> regex::Regex {
    Regex::new(r"pi(?P<pid>[1-3])").unwrap()
}

pub fn int_const() -> regex::Regex {
    Regex::new(r"ci(?P<n>-?[0-9]*)").unwrap()
}

pub fn array_variable() -> regex::Regex {
    Regex::new(r"va(?P<vid>[1-5])").unwrap()
}

pub fn array_parameter() -> regex::Regex {
    Regex::new(r"pa(?P<pid>[1-3])").unwrap()
}

pub fn int_variable_def() -> regex::Regex {
    Regex::new(r"^var").unwrap()
}

pub fn array_variable_def() -> regex::Regex {
    Regex::new(r"^vet").unwrap()
}

#[derive(Debug)]
pub struct PatchAct {
    pub byte_pos: u32,
    pub patch: Vec<u8>,
}

#[derive(Debug)]
pub struct FunctionAct {
    pub name: String,
    pub params: Vec<ParamAct>,
    pub body_start: u32,
}

#[derive(Debug)]
pub struct MethodAct {
    pub function: FunctionAct,
}

#[derive(Debug)]
pub struct ClassAct {
    pub name: String,
    // TODO: constructor
    pub methods: Vec<MethodAct>,
}

#[derive(Debug, PartialEq)]
pub enum TypeAct {
    Number,
    String,
    Unknown,
}
#[derive(Debug)]
pub struct ParamAct {
    pub name: String,
    pub act_type: TypeAct,
}

pub fn get_ts_type_from_acttype(act_type: &TypeAct) -> String {
    match act_type {
        TypeAct::Number => "number".to_string(),
        TypeAct::String => "string".to_string(),
        TypeAct::Unknown => "unknown".to_string(),
    }
}

pub fn get_js_constructor_from_acttype(act_type: &TypeAct) -> String {
    match act_type {
        TypeAct::Number => "Number".to_string(),
        TypeAct::String => "String".to_string(),
        TypeAct::Unknown => "".to_string(),
    }
}

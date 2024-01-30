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
    pub methods: Vec<MethodAct>,
}

#[derive(Debug, PartialEq)]
pub enum TypeAct {
    Number,
    String,
    BigInt,
    Boolean,
    Symbol,
    Float32Array,
    Float64Array,
    Int8Array,
    Int16Array,
    Int32Array,
    Uint8Array,
    Uint8ClampedArray,
    Uint16Array,
    Uint32Array,
    BigInt64Array,
    BigUint64Array,
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
        TypeAct::BigInt => "bigint".to_string(),
        TypeAct::Boolean => "boolean".to_string(),
        TypeAct::Symbol => "symbol".to_string(),
        TypeAct::Float32Array => "Float32Array".to_string(),
        TypeAct::Float64Array => "Float64Array".to_string(),
        TypeAct::Int8Array => "Int8Array".to_string(),
        TypeAct::Int16Array => "Int16Array".to_string(),
        TypeAct::Int32Array => "Int32Array".to_string(),
        TypeAct::Uint8Array => "Uint8Array".to_string(),
        TypeAct::Uint8ClampedArray => "Uint8ClampedArray".to_string(),
        TypeAct::Uint16Array => "Uint16Array".to_string(),
        TypeAct::Uint32Array => "Uint32Array".to_string(),
        TypeAct::BigInt64Array => "BigInt64Array".to_string(),
        TypeAct::BigUint64Array => "BigUint64Array".to_string(),
        TypeAct::Unknown => "unknown".to_string(),
    }
}

pub fn get_js_constructor_from_acttype(act_type: &TypeAct) -> String {
    match act_type {
        TypeAct::Number => "Number".to_string(),
        TypeAct::String => "String".to_string(),
        TypeAct::BigInt => "BigInt".to_string(),
        TypeAct::Boolean => "Boolean".to_string(),
        TypeAct::Symbol => "Symbol".to_string(),
        TypeAct::Float32Array => "new Float32Array".to_string(),
        TypeAct::Float64Array => "new Float64Array".to_string(),
        TypeAct::Int8Array => "new Int8Array".to_string(),
        TypeAct::Int16Array => "new Int16Array".to_string(),
        TypeAct::Int32Array => "new Int32Array".to_string(),
        TypeAct::Uint8Array => "new Uint8Array".to_string(),
        TypeAct::Uint8ClampedArray => "new Uint8ClampedArray".to_string(),
        TypeAct::Uint16Array => "new Uint16Array".to_string(),
        TypeAct::Uint32Array => "new Uint32Array".to_string(),
        TypeAct::BigInt64Array => "new BigInt64Array".to_string(),
        TypeAct::BigUint64Array => "new BigUint64Array".to_string(),
        TypeAct::Unknown => "".to_string(),
    }
}

pub fn get_acttype_from_string(type_str: &str) -> TypeAct {
    match type_str {
        "number" => TypeAct::Number,
        "string" => TypeAct::String,
        "bigint" => TypeAct::BigInt,
        "boolean" => TypeAct::Boolean,
        "symbol" => TypeAct::Symbol,
        "object" => TypeAct::Unknown,
        "unknown" => TypeAct::Unknown,
        "Float32Array" => TypeAct::Float32Array,
        "Float64Array" => TypeAct::Float64Array,
        "Int8Array" => TypeAct::Int8Array,
        "Int16Array" => TypeAct::Int16Array,
        "Int32Array" => TypeAct::Int32Array,
        "Uint8Array" => TypeAct::Uint8Array,
        "Uint8ClampedArray" => TypeAct::Uint8ClampedArray,
        "Uint16Array" => TypeAct::Uint16Array,
        "Uint32Array" => TypeAct::Uint32Array,
        "BigInt64Array" => TypeAct::BigInt64Array,
        "BigUint64Array" => TypeAct::BigUint64Array,
        _ => TypeAct::Unknown,
    }
}

pub fn get_typeinfo_operator_from_acttype(act_type: &TypeAct) -> String {
    match act_type {
        TypeAct::Number => "typeof".to_string(),
        TypeAct::String => "typeof".to_string(),
        TypeAct::BigInt => "typeof".to_string(),
        TypeAct::Boolean => "typeof".to_string(),
        TypeAct::Symbol => "typeof".to_string(),
        TypeAct::Float32Array => "instanceof".to_string(),
        TypeAct::Float64Array => "instanceof".to_string(),
        TypeAct::Int8Array => "instanceof".to_string(),
        TypeAct::Int16Array => "instanceof".to_string(),
        TypeAct::Int32Array => "instanceof".to_string(),
        TypeAct::Uint8Array => "instanceof".to_string(),
        TypeAct::Uint8ClampedArray => "instanceof".to_string(),
        TypeAct::Uint16Array => "instanceof".to_string(),
        TypeAct::Uint32Array => "instanceof".to_string(),
        TypeAct::BigInt64Array => "instanceof".to_string(),
        TypeAct::BigUint64Array => "instanceof".to_string(),
        TypeAct::Unknown => "".to_string(),
    }
}

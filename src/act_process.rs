use std::{path::PathBuf, println, vec};

use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_ast::{ClassDecl, FnDecl, Function, ModuleItem, Param, TsKeywordTypeKind};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

use crate::{
    act_patch::{apply_patches, get_function_params_patches},
    act_structs::{ClassAct, FunctionAct, MethodAct, ParamAct, PatchAct, TypeAct},
};

pub fn get_typeact_from_typeid(typeid: TsKeywordTypeKind) -> TypeAct {
    if typeid == TsKeywordTypeKind::TsNumberKeyword {
        return TypeAct::Number;
    };
    if typeid == TsKeywordTypeKind::TsStringKeyword {
        return TypeAct::String;
    };

    return TypeAct::Unknown;
}

pub fn get_param_type_id(param: &Param) -> TsKeywordTypeKind {
    let param_type_ann = param
        .clone()
        .pat
        .ident()
        .unwrap()
        .type_ann
        .unwrap()
        .type_ann;
    if param_type_ann.is_ts_keyword_type() {
        return param_type_ann.ts_keyword_type().unwrap().kind;
    } else {
        return TsKeywordTypeKind::TsUnknownKeyword;
    }
}
pub fn get_function_params(params: Vec<Param>) -> Vec<ParamAct> {
    let mut params_act: Vec<ParamAct> = vec![];
    for param in params {
        let param_type_id = get_param_type_id(&param);
        let param_name = param.pat.ident().unwrap().sym.to_string();
        params_act.push(ParamAct {
            name: param_name,
            act_type: get_typeact_from_typeid(param_type_id),
        })
    }
    return params_act;
}

pub fn get_function_act(function_name: String, function: Box<Function>) -> FunctionAct {
    let function_body_start = function.body.unwrap().span.lo.0;
    let function_act: FunctionAct = FunctionAct {
        name: function_name,
        params: get_function_params(function.params),
        body_start: function_body_start,
    };
    return function_act;
}

pub fn get_function_patches(function_act: FunctionAct) -> Vec<PatchAct> {
    let mut patches: Vec<PatchAct> = vec![];
    patches.extend(get_function_params_patches(
        function_act.params,
        function_act.body_start,
    ));
    return patches;
}

pub fn process_function(fn_decl: FnDecl) -> Vec<PatchAct> {
    let function_name = fn_decl.ident.sym.to_string();
    let function_act = get_function_act(function_name, fn_decl.function);
    let function_patches: Vec<PatchAct> = get_function_patches(function_act);
    return function_patches;
}

pub fn get_class_act(class_decl: ClassDecl) -> ClassAct {
    let class_name = class_decl.ident.sym.to_string();
    let class = class_decl.class;
    let class_props = class.body;
    let mut methods_act: Vec<MethodAct> = vec![];
    for class_prop in class_props {
        if class_prop.is_method() {
            let method = class_prop.method().unwrap();
            let function_act = get_function_act("TODO".to_owned(), method.function);
            let method_act: MethodAct = MethodAct {
                function: function_act,
            };
            methods_act.push(method_act)
        }
    }
    let class_act: ClassAct = ClassAct {
        name: class_name,
        methods: methods_act,
    };
    return class_act;
}

pub fn get_class_patches(class_act: ClassAct) -> Vec<PatchAct> {
    let mut patches: Vec<PatchAct> = vec![];
    // patches.extend(get_constructor_patches(&class_act));
    patches.extend(get_methods_patches(class_act));
    return patches;
}

fn get_constructor_patches(class_act: &ClassAct) -> Vec<PatchAct> {
    // TODO: get constructor patch
    vec![]
}

fn get_methods_patches(class_act: ClassAct) -> Vec<PatchAct> {
    let mut patches: Vec<PatchAct> = vec![];
    for method in class_act.methods {
        patches.extend(get_function_params_patches(
            method.function.params,
            method.function.body_start,
        ));
    }
    return patches;
}

pub fn process_class(class_decl: ClassDecl) -> Vec<PatchAct> {
    let class_act = get_class_act(class_decl);
    let class_patches: Vec<PatchAct> = get_class_patches(class_act);
    return class_patches;
}

pub fn process_module_items(module_items: Vec<ModuleItem>) -> Vec<PatchAct> {
    let mut patches: Vec<PatchAct> = vec![];
    for item in module_items {
        if item.is_stmt() {
            let stmt = item.stmt().unwrap();
            if stmt.is_decl() {
                let decl = stmt.decl().unwrap();
                if decl.is_fn_decl() {
                    let fn_decl = decl.fn_decl().unwrap();
                    patches.extend(process_function(fn_decl));
                } else if decl.is_class() {
                    let class_decl = decl.class().unwrap();
                    patches.extend(process_class(class_decl));
                }
            }
        }
    }
    return patches;
}

pub fn process_file(file_path: PathBuf) -> Result<(), String> {
    println!("analysing file {}", file_path.to_str().unwrap());
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.load_file(&file_path).expect("failed to load ts file");
    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let module = parser
        .parse_typescript_module()
        .expect("failed to parser module");
    let patches = process_module_items(module.body);

    apply_patches(patches, file_path).unwrap();

    Ok(())
}
use std::path::PathBuf;

use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_ast::{FnDecl, ModuleItem, Param, TsKeywordTypeKind};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

use crate::{
    act_patch::{apply_patches, get_function_params_patches},
    act_structs::{FunctionAct, ParamAct, PatchAct, TypeAct},
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

pub fn get_function_act(fn_decl: FnDecl) -> FunctionAct {
    let function_name = fn_decl.ident.sym.to_string();
    let function = fn_decl.function;
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
    let function_act = get_function_act(fn_decl);
    let function_patches: Vec<PatchAct> = get_function_patches(function_act);
    return function_patches;
}

pub fn process_module_items(module_items: Vec<ModuleItem>) -> Vec<PatchAct> {
    let mut function_patches: Vec<PatchAct> = vec![];
    for item in module_items {
        if item.is_stmt() {
            let stmt = item.stmt().unwrap();
            if stmt.is_decl() {
                let decl = stmt.decl().unwrap();
                if decl.is_fn_decl() {
                    let fn_decl = decl.fn_decl().unwrap();
                    function_patches.extend(process_function(fn_decl));
                }
            }
        }
    }
    return function_patches;
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


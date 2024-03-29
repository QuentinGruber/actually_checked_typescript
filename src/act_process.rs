use std::path::{Path, PathBuf};
use std::{println, vec};

use swc_common::{sync::Lrc, Span};
use swc_common::{BytePos, SourceMap, SyntaxContext};
use swc_ecma_ast::{
    ArrowExpr, ClassDecl, Decl, EsVersion, FnDecl, FnExpr, Function, ModuleItem, Param, Pat,
    TsKeywordType, TsKeywordTypeKind, TsType, VarDecl,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

use crate::act_structs::get_acttype_from_string;
use crate::{
    act_patch::{apply_patches, get_function_params_patches},
    act_structs::{ClassAct, FunctionAct, MethodAct, ParamAct, PatchAct, TypeAct},
};

pub fn get_typeact_from_typeid(typeid: TsKeywordTypeKind) -> TypeAct {
    match typeid {
        TsKeywordTypeKind::TsBooleanKeyword => TypeAct::Boolean,
        TsKeywordTypeKind::TsNumberKeyword => TypeAct::Number,
        TsKeywordTypeKind::TsStringKeyword => TypeAct::String,
        TsKeywordTypeKind::TsUnknownKeyword => TypeAct::Unknown,
        TsKeywordTypeKind::TsBigIntKeyword => TypeAct::BigInt,
        TsKeywordTypeKind::TsSymbolKeyword => TypeAct::Symbol,
        _ => TypeAct::Unknown,
    }
}

pub fn get_param_type_ann(param_pat: &Pat) -> Result<Box<TsType>, String> {
    // FIXME: This is a hack, we should not clone
    let param_pat = param_pat.clone();
    let mut param_type_ann: Box<TsType> = Box::new(TsType::TsKeywordType(TsKeywordType {
        span: Span {
            lo: BytePos::DUMMY,
            hi: BytePos::DUMMY,
            ctxt: SyntaxContext::default(),
        },
        kind: TsKeywordTypeKind::TsUnknownKeyword,
    }));
    if param_pat.is_ident() {
        let param_ident = param_pat.ident().unwrap();
        if param_ident.optional {
            // If is optionnal, we skip it
            return Ok(param_type_ann);
        }
        if param_ident.type_ann.is_none() {
            return Err(String::from("param_ident.type_ann.is_none()"));
        }
        let param_type_ann_wraped = param_ident.type_ann.unwrap();
        param_type_ann = param_type_ann_wraped.type_ann;
    } else if param_pat.is_expr() {
        let _param_expr = param_pat.expr().unwrap();
        // TODO:
    }

    Ok(param_type_ann)
}
pub fn get_param_type_act(param_pat: &Pat) -> TypeAct {
    let param_type_ann = get_param_type_ann(param_pat).unwrap();
    if param_type_ann.is_ts_keyword_type() {
        get_typeact_from_typeid(param_type_ann.ts_keyword_type().unwrap().kind)
    } else if param_type_ann.is_ts_type_ref() {
        let type_ref = param_type_ann.ts_type_ref().unwrap();
        if type_ref.type_name.is_ident() {
            let type_ref_type_name = type_ref.type_name.ident().unwrap().sym.to_string();
            get_acttype_from_string(&type_ref_type_name)
        } else {
            TypeAct::Unknown
        }
    } else {
        TypeAct::Unknown
    }
}

fn get_param_name(param_pat: Pat) -> String {
    if param_pat.is_ident() {
        param_pat.ident().unwrap().sym.to_string()
    } else {
        "unknown".to_string()
    }
}
pub fn get_function_params(params: Vec<Pat>) -> Vec<ParamAct> {
    let mut params_act: Vec<ParamAct> = vec![];
    for param in params {
        let param_type_act = get_param_type_act(&param);
        let param_name = get_param_name(param);
        params_act.push(ParamAct {
            name: param_name,
            act_type: param_type_act,
        })
    }
    params_act
}

pub fn get_function_act(function_name: String, function: Box<Function>) -> FunctionAct {
    if function.body.is_none() {
        panic!("Function body is empty get_function_act should not be called");
    }
    let function_body = function.body.unwrap();
    let function_body_start = function_body.span.lo.0;
    let function_act: FunctionAct = FunctionAct {
        name: function_name,
        params: get_function_params(get_pat_from_param(function.params)),
        body_start: function_body_start,
    };
    function_act
}

pub fn get_function_patches(function_act: FunctionAct, file_name: &Path) -> Vec<PatchAct> {
    let mut patches: Vec<PatchAct> = vec![];
    patches.extend(get_function_params_patches(
        function_act.params,
        function_act.body_start,
        function_act.name,
        file_name.to_str().unwrap().to_string(),
    ));
    patches
}

pub fn process_function_decl(fn_decl: FnDecl, file_path: &Path) -> Vec<PatchAct> {
    let function_name = fn_decl.ident.sym.to_string();
    if fn_decl.function.body.is_some() {
        let function_act = get_function_act(function_name, fn_decl.function);
        let function_patches: Vec<PatchAct> = get_function_patches(function_act, file_path);
        function_patches
    } else {
        vec![]
    }
}
pub fn process_var_decl(var_decl: Box<VarDecl>, file_path: &Path) -> Vec<PatchAct> {
    let var_decl_decls = var_decl.decls;
    let mut patches: Vec<PatchAct> = vec![];
    for var_decl_decl in var_decl_decls {
        let var_decl_decl_name = var_decl_decl.name;
        let var_decl_decl_init = var_decl_decl.init;
        if var_decl_decl_init.is_some() {
            let var_decl_decl_init_wraped = var_decl_decl_init.unwrap();
            if var_decl_decl_init_wraped.is_fn_expr() {
                let fn_expr = var_decl_decl_init_wraped.fn_expr().unwrap();
                if var_decl_decl_name.is_ident() {
                    let function_name = var_decl_decl_name.ident().unwrap().sym.to_string();
                    let function_act = get_function_act(function_name, fn_expr.function);
                    patches.extend(get_function_patches(function_act, file_path));
                } else {
                    let function_name = "unknonVarName".to_string();
                    let function_act = get_function_act(function_name, fn_expr.function);
                    patches.extend(get_function_patches(function_act, file_path));
                }
            } else if var_decl_decl_init_wraped.is_arrow() {
                let arrow_expr = var_decl_decl_init_wraped.arrow().unwrap();
                let function_body = arrow_expr.body;
                if function_body.is_block_stmt() {
                    if var_decl_decl_name.is_ident() {
                        let function_name = var_decl_decl_name.ident().unwrap().sym.to_string();
                        let function_body_block_stmt = function_body.block_stmt().unwrap();
                        let function_body_start = function_body_block_stmt.span.lo.0;
                        let function_act: FunctionAct = FunctionAct {
                            name: function_name,
                            params: get_function_params(arrow_expr.params),
                            body_start: function_body_start,
                        };
                        let function_patches: Vec<PatchAct> =
                            get_function_patches(function_act, file_path);
                        patches.extend(function_patches);
                    } else {
                        let function_name = "unknonVarName".to_string();
                        let function_body_block_stmt = function_body.block_stmt().unwrap();
                        let function_body_start = function_body_block_stmt.span.lo.0;
                        let function_act: FunctionAct = FunctionAct {
                            name: function_name,
                            params: get_function_params(arrow_expr.params),
                            body_start: function_body_start,
                        };
                        let function_patches: Vec<PatchAct> =
                            get_function_patches(function_act, file_path);
                        patches.extend(function_patches);
                    }
                }
            }
        }
    }
    patches
}
pub fn process_function_expr(fn_expr: FnExpr, file_path: &Path) -> Vec<PatchAct> {
    let function_name = fn_expr.ident.unwrap().sym.to_string();
    if fn_expr.function.body.is_some() {
        let function_act = get_function_act(function_name, fn_expr.function);
        let function_patches: Vec<PatchAct> = get_function_patches(function_act, file_path);
        function_patches
    } else {
        vec![]
    }
}

pub fn process_function_arrow(arrow_expr: ArrowExpr, file_path: &Path) -> Vec<PatchAct> {
    let function_body = arrow_expr.body;
    if function_body.is_block_stmt() {
        let function_body_block_stmt = function_body.block_stmt().unwrap();
        let function_body_start = function_body_block_stmt.span.lo.0;
        let function_act: FunctionAct = FunctionAct {
            name: "AnonymousFunction".to_string(),
            params: get_function_params(arrow_expr.params),
            body_start: function_body_start,
        };
        let function_patches: Vec<PatchAct> = get_function_patches(function_act, file_path);
        return function_patches;
    }
    vec![]
}

pub fn get_class_act(class_decl: ClassDecl) -> ClassAct {
    let class_name = class_decl.ident.sym.to_string();
    let class = class_decl.class;
    let class_props = class.body;
    let mut methods_act: Vec<MethodAct> = vec![];
    for class_prop in class_props {
        if class_prop.is_method() {
            let method = class_prop.method().unwrap();
            let method_key = method.key;
            let mut method_name: String = "unknownName".to_string();
            if method_key.is_ident() {
                let method_key_ident = method_key.ident().unwrap();
                method_name = method_key_ident.sym.to_string();
            }
            if method.function.body.is_some() {
                let function_act = get_function_act(method_name, method.function);
                let method_act: MethodAct = MethodAct {
                    function: function_act,
                };
                methods_act.push(method_act)
            }
        } else if class_prop.is_constructor() {
            let constructor = class_prop.constructor().unwrap();
            if constructor.params.is_empty() {
                continue;
            }
            if constructor.body.is_some() {
                let constructor_body = constructor.body.unwrap();
                let constructor_body_start = constructor_body.span.lo.0;
                let mut params: Vec<Param> = vec![];
                for param in constructor.params {
                    if param.is_param() {
                        params.push(param.param().unwrap())
                    }
                }
                let constructor_act: MethodAct = MethodAct {
                    function: FunctionAct {
                        name: "constructor".to_string(),
                        params: get_function_params(get_pat_from_param(params)),
                        body_start: constructor_body_start,
                    },
                };
                methods_act.push(constructor_act)
            }
        }
    }
    let class_act: ClassAct = ClassAct {
        name: class_name,
        methods: methods_act,
    };
    class_act
}

pub fn get_pat_from_param(params: Vec<Param>) -> Vec<Pat> {
    let mut pats: Vec<Pat> = vec![];
    for param in params {
        pats.push(param.pat)
    }
    pats
}

pub fn get_class_patches(class_act: ClassAct, file_path: &Path) -> Vec<PatchAct> {
    let mut patches: Vec<PatchAct> = vec![];
    patches.extend(get_methods_patches(class_act, file_path));
    patches
}

fn get_methods_patches(class_act: ClassAct, file_path: &Path) -> Vec<PatchAct> {
    let mut patches: Vec<PatchAct> = vec![];
    for method in class_act.methods {
        patches.extend(get_function_params_patches(
            method.function.params,
            method.function.body_start,
            method.function.name,
            file_path.to_str().unwrap().to_string(),
        ));
    }
    patches
}

pub fn process_class_decl(class_decl: ClassDecl, file_path: &Path) -> Vec<PatchAct> {
    let class_act = get_class_act(class_decl);
    let class_patches: Vec<PatchAct> = get_class_patches(class_act, file_path);
    class_patches
}

pub fn process_decl(decl: Decl, file_path: &Path) -> Vec<PatchAct> {
    if decl.is_fn_decl() {
        let fn_decl = decl.fn_decl().unwrap();
        process_function_decl(fn_decl, file_path)
    } else if decl.is_class() {
        let class_decl = decl.class().unwrap();
        return process_class_decl(class_decl, file_path);
    } else if decl.is_var() {
        return process_var_decl(decl.var().unwrap(), file_path);
    } else {
        return vec![];
    }
}

pub fn process_module_items(
    module_items: Vec<ModuleItem>,
    file_path: &Path,
) -> Result<Vec<PatchAct>, String> {
    let mut patches: Vec<PatchAct> = vec![];
    for item in module_items {
        if item.is_stmt() {
            let stmt = item.stmt().unwrap();
            if stmt.is_decl() {
                let decl = stmt.decl().unwrap();
                patches.extend(process_decl(decl, file_path));
            } else if stmt.is_expr() {
                let expr = stmt.expr().unwrap().expr;
                if expr.is_fn_expr() {
                    let fn_expr = expr.fn_expr().unwrap();
                    patches.extend(process_function_expr(fn_expr, file_path));
                } else if expr.is_arrow() {
                    let arrow_expr = expr.arrow().unwrap();
                    patches.extend(process_function_arrow(arrow_expr, file_path));
                }
            }
        } else if item.is_module_decl() {
            let module_decl = item.module_decl().unwrap();
            if module_decl.is_export_decl() {
                let export_decl = module_decl.export_decl().unwrap();
                let decl = export_decl.decl;
                patches.extend(process_decl(decl, file_path));
            }
        }
    }
    Ok(patches)
}

pub fn process_file(file_path: PathBuf) -> Result<(), String> {
    println!("analysing file {}", file_path.to_str().unwrap());
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.load_file(&file_path).expect("failed to load ts file");
    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            decorators: true,
            tsx: false,
            disallow_ambiguous_jsx_like: true,
            no_early_errors: true,
            dts: false,
        }),
        EsVersion::EsNext,
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let module_wrap = parser.parse_typescript_module();
    let mut patches: Vec<PatchAct> = vec![];
    if module_wrap.is_ok() {
        let module = module_wrap.unwrap();
        let module_item_wrap = process_module_items(module.body, &file_path);
        if module_item_wrap.is_ok() {
            patches = module_item_wrap.unwrap();
        } else if module_item_wrap.is_err() {
            println!("error processing file {}", file_path.to_str().unwrap());
            let err = module_item_wrap.err().unwrap();
            println!("{:?}", err);
        }
    } else if module_wrap.is_err() {
        println!("error parsing file {}", file_path.to_str().unwrap());
        let err = module_wrap.err().unwrap();
        println!("{:?}", err.into_kind());
    }

    apply_patches(patches, file_path).unwrap();

    Ok(())
}

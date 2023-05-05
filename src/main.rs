use std::{fs, path::PathBuf, thread::spawn};

use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_ast::{FnDecl, ModuleItem, Param, TsKeywordTypeKind};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

fn get_files_paths(folder_path: String) -> Vec<PathBuf> {
    let files = fs::read_dir(folder_path)
        .expect("Unable to read directory")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
                && entry.path().extension().unwrap_or_default() == "ts"
                && !entry.path().to_str().unwrap().contains(".checked")
        })
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    return files;
}

#[derive(Debug)]
struct PatchAct {
    byte_pos: u32,
    patch: Vec<u8>,
}

#[derive(Debug)]
struct FunctionAct {
    name: String,
    params: Vec<ParamAct>,
    body_start: u32,
}

#[derive(Debug, PartialEq)]
enum TypeAct {
    Number,
    String,
    Unknown,
}
#[derive(Debug)]
struct ParamAct {
    name: String,
    act_type: TypeAct,
}

fn get_typeact_from_typeid(typeid: TsKeywordTypeKind) -> TypeAct {
    if typeid == TsKeywordTypeKind::TsNumberKeyword {
        return TypeAct::Number;
    };
    if typeid == TsKeywordTypeKind::TsStringKeyword {
        return TypeAct::String;
    };

    return TypeAct::Unknown;
}

fn get_param_type_id(param: &Param) -> TsKeywordTypeKind {
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
fn get_function_params(params: Vec<Param>) -> Vec<ParamAct> {
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

fn get_function_act(fn_decl: FnDecl) -> FunctionAct {
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

fn get_function_patches(function_act: FunctionAct) -> Vec<PatchAct> {
    let mut patches: Vec<PatchAct> = vec![];
    patches.extend(get_function_params_patches(
        function_act.params,
        function_act.body_start,
    ));
    return patches;
}

fn get_tstype_from_acttype(act_type: TypeAct) -> String {
    match act_type {
        TypeAct::Number => "number".to_string(),
        TypeAct::String => "string".to_string(),
        TypeAct::Unknown => "unknown".to_string(),
    }
}

fn get_function_param_patch(param: ParamAct, body_start: u32) -> PatchAct {
    let param_ts_type = get_tstype_from_acttype(param.act_type);
    let patch_string = format!(
        r#"
    if(typeof {} !== '{}'){{
      throw `{} isn't of type {} but of type ${{typeof {}}}`
    }}
    "#,
        param.name, param_ts_type, param.name, param_ts_type, param.name
    );
    return PatchAct {
        byte_pos: body_start,
        patch: patch_string.as_bytes().to_vec(),
    };
}

fn get_function_params_patches(params: Vec<ParamAct>, body_start: u32) -> Vec<PatchAct> {
    let mut params_patches: Vec<PatchAct> = vec![];
    for param in params
        .into_iter()
        .filter(|x| x.act_type != TypeAct::Unknown)
    {
        params_patches.push(get_function_param_patch(param, body_start));
    }
    return params_patches;
}

fn apply_patches(patches: Vec<PatchAct>, file_path: PathBuf) -> Result<(), String> {
    let mut buffer = fs::read(&file_path).unwrap_or_default();
    // TODO: faut penser aux multiple patch qui decale l'index genre faudrais faire une hashmap /
    // datastructure qui permet de savoir combien faut decal entre chaque patch en fonction de
    // l'index a modifier
    for patch in patches {
        let pos: usize = patch.byte_pos as usize;
        buffer.splice(pos..pos, patch.patch);
    }
    let patched_file_path: PathBuf = file_path.with_extension("checked.ts");
    fs::write(patched_file_path, buffer).unwrap();
    Ok(())
}
fn process_function(fn_decl: FnDecl) -> Vec<PatchAct> {
    let function_act = get_function_act(fn_decl);
    let function_patches: Vec<PatchAct> = get_function_patches(function_act);
    return function_patches;
}

fn process_module_items(module_items: Vec<ModuleItem>) -> Vec<PatchAct> {
    let mut function_patches: Vec<PatchAct> = vec![];
    for item in module_items {
        let stmt = item.stmt().unwrap();
        if stmt.is_decl() {
            let decl = stmt.decl().unwrap();
            if decl.is_fn_decl() {
                let fn_decl = decl.fn_decl().unwrap();
                function_patches.extend(process_function(fn_decl));
            }
        }
    }
    return function_patches;
}

fn process_file(file_path: PathBuf) -> Result<(), String> {
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

fn main() {
    let folder_path = std::env::args().nth(1).expect("No folder path provided");
    let files = get_files_paths(folder_path);
    for file_path in files {
        spawn(move || process_file(file_path).unwrap())
            .join()
            .unwrap();
    }
}

//TODO: test buffer unitaire
#[cfg(test)]
mod tests {

    #[test]
    fn mdr_test() {
        assert_eq!(1, 1)
    }
}

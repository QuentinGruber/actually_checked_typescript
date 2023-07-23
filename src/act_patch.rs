use std::{fs, path::PathBuf, str::FromStr};

use crate::{
    act_structs::{
        get_js_constructor_from_acttype, get_ts_type_from_acttype, ParamAct, PatchAct, TypeAct,
    },
    args_parser::ActArgs,
    patch_index_helper::PatchIndexHelper,
};
use clap::Parser;

#[derive(Debug, Clone)]
pub enum PatchType {
    Warning,
    Error,
    Fix,
}
impl FromStr for PatchType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "warning" => Ok(PatchType::Warning),
            "error" => Ok(PatchType::Error),
            "fix" => Ok(PatchType::Fix),
            _ => Err(String::from("Invalid enum value")),
        }
    }
}

pub fn gen_param_type_check_patch(param: ParamAct) -> String {
    let args = ActArgs::parse();
    let patch_type = args.patch_type;
    let param_ts_type = get_ts_type_from_acttype(&param.act_type);
    let param_js_constructor = get_js_constructor_from_acttype(&param.act_type);
    let log_message = format!(
        r#"`{} isn't of type {} but of type ${{typeof {}}}`"#,
        param.name, param_ts_type, param.name
    );
    let patch_body = match patch_type {
        PatchType::Fix => format!(
            r#"console.warn({}," and was casted"); {}({});"#,
            log_message, param_js_constructor, param.name
        ),
        PatchType::Error => format!(r#"throw {};"#, log_message),
        PatchType::Warning => format!(r#"console.warn({});"#, log_message),
    };
    let patch_string = format!(
        r#"
    if(typeof {} !== '{}'){{
    {}
    }}
    "#,
        param.name, param_ts_type, patch_body
    );
    patch_string
}

pub fn get_function_param_patch(param: ParamAct, body_start: u32) -> PatchAct {
    let patch_string = gen_param_type_check_patch(param);
    return PatchAct {
        byte_pos: body_start,
        patch: patch_string.as_bytes().to_vec(),
    };
}

pub fn get_function_params_patches(params: Vec<ParamAct>, body_start: u32) -> Vec<PatchAct> {
    let mut params_patches: Vec<PatchAct> = vec![];
    for param in params
        .into_iter()
        .filter(|x| x.act_type != TypeAct::Unknown)
    {
        params_patches.push(get_function_param_patch(param, body_start));
    }
    params_patches
}

pub fn apply_patches(patches: Vec<PatchAct>, file_path: PathBuf) -> Result<(), String> {
    let mut buffer = fs::read(&file_path).unwrap_or_default();
    let mut patch_index_helper = PatchIndexHelper::new();
    for patch in patches {
        let pos: usize = patch_index_helper.get_drifted_index(patch.byte_pos) as usize;
        let patch_len: u32 = patch.patch.len() as u32;
        buffer.splice(pos..pos, patch.patch);
        patch_index_helper.register_patched_index(patch.byte_pos, patch_len)
    }
    let args = ActArgs::parse();
    let out_folder_path = args.out_folder_path;
    let in_folder_path = args.folder_path;

    let mut patched_file_path = file_path.clone();
    // remove the first
    patched_file_path = patched_file_path
        .strip_prefix(&in_folder_path)
        .unwrap()
        .to_path_buf();
    patched_file_path = PathBuf::from(out_folder_path).join(patched_file_path);
    let patch_file_path_without_filename = patched_file_path
        .parent()
        .unwrap_or_else(|| {
            println!("Fail to get parent of {:?}", patched_file_path);
            panic!("Fail to get parent of {:?}", patched_file_path);
        })
        .to_path_buf();
    fs::create_dir_all(patch_file_path_without_filename).unwrap_or_else(|err| {
        println!("{:?}", err);
        panic!("Fail to create out_folder_path");
    });
    fs::write(patched_file_path, buffer).unwrap();
    Ok(())
}

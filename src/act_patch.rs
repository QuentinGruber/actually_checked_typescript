use std::{fs, path::PathBuf};

use crate::{
    act_structs::{get_tstype_from_acttype, ParamAct, PatchAct, TypeAct},
    patch_index_helper::PatchIndexHelper,
};

#[derive(Debug)]
pub enum PatchType {
    Warning,
    Error,
    Fix,
}

pub fn gen_param_type_check_patch(param: ParamAct, patch_type: PatchType) -> String {
    let param_ts_type = get_tstype_from_acttype(param.act_type);
    let log_message = format!(
        r#"`{} isn't of type {} but of type ${{typeof {}}}`"#,
        param.name, param_ts_type, param.name
    );
    let patch_body = match patch_type {
        PatchType::Fix => format!(r#"console.warn({})"#, log_message), // TODO
        PatchType::Error => format!(r#"throw {}"#, log_message),
        PatchType::Warning => format!(r#"console.warn({})"#, log_message),
    };
    let patch_string = format!(
        r#"
    if(typeof {} !== '{}'){{
    {}
    }}
    "#,
        param.name, param_ts_type, patch_body
    );
    return patch_string;
}

pub fn get_function_param_patch(param: ParamAct, body_start: u32) -> PatchAct {
    let patch_string = gen_param_type_check_patch(param, PatchType::Warning);
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
    return params_patches;
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
    let patched_file_path: PathBuf = file_path.with_extension("checked.ts");
    fs::write(patched_file_path, buffer).unwrap();
    Ok(())
}

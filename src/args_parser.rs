use clap::Parser;

use crate::act_patch::PatchType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ActArgs {
    #[arg(short, long, default_value = ".")]
    pub folder_path: String,

    #[arg(short, long, default_value = ".")]
    pub out_folder_path: String,

    #[arg(value_enum, default_value_t = PatchType::Warning)]
    pub patch_type: PatchType,
}

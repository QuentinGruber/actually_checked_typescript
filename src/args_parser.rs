use clap::Parser;

use crate::act_patch::PatchType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ActArgs {
    #[arg(short, long)]
    pub folder_path: String,

    #[arg(short, long, default_value = "act_out")]
    // TODO make it optional and default to /act_out
    pub out_folder_path: String,

    #[arg(short, long)]
    pub patch_type: PatchType,
}

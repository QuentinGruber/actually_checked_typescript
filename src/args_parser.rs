use clap::Parser;

use crate::act_patch::PatchType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ActArgs {
    #[arg(short, long)]
    pub folder_path: String,

    #[arg(short, long)]
    pub patch_type: PatchType,
}

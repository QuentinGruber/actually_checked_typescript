use act_lib::{act_process::process_file, args_parser::ActArgs};
use clap::Parser;
use std::{fs, path::PathBuf, thread::spawn};

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

fn main() {
    let args = ActArgs::parse();
    let folder_path = args.folder_path;
    let files = get_files_paths(folder_path);
    for file_path in files {
        spawn(move || process_file(file_path).unwrap_or(()))
            .join()
            .unwrap_or(());
    }
}

//TODO: test fonctionnel
#[cfg(test)]
mod tests {

    #[test]
    fn mdr_test() {
        assert_eq!(1, 1)
    }
}

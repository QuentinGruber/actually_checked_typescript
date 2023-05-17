use act_lib::{act_process::process_file, args_parser::ActArgs};
use clap::Parser;
use std::{fs, path::PathBuf, println, thread};

fn get_files_paths(folder_path: String) -> Vec<PathBuf> {
    let files = fs::read_dir(folder_path).expect("Unable to read directory");
    let mut files_to_process: Vec<PathBuf> = vec![];
    for file in files {
        let entry = file.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file() {
            let is_ts_file = entry.path().extension().unwrap_or_default() == "ts";
            // TODO: remove
            let is_not_checked = !entry.path().to_str().unwrap().contains(".checked");
            if is_ts_file && is_not_checked {
                files_to_process.push(entry.path())
            }
        } else if file_type.is_dir() {
            files_to_process.extend(get_files_paths(entry.path().to_str().unwrap().to_string()));
        }
    }
    return files_to_process;
}

fn main() {
    let args = ActArgs::parse();
    let folder_path = args.folder_path;
    let files = get_files_paths(folder_path);
    for file_path in files {
        thread::Builder::new()
            .name(file_path.to_string_lossy().to_string())
            .spawn(move || process_file(file_path).unwrap_or(()))
            .unwrap_or_else(|err| {
                println!("{:?}", err);
                panic!();
            })
            .join()
            .unwrap_or_else(|err| {
                println!("{:?}", err);
            });
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

use act_lib::{act_process::process_file, args_parser::ActArgs};
use clap::Parser;
use std::{fs, path::PathBuf, println, thread, time::Instant};

fn get_files_paths(folder_path: String) -> Vec<PathBuf> {
    let files = fs::read_dir(folder_path).expect("Unable to read directory");
    let mut files_to_process: Vec<PathBuf> = vec![];
    for file in files {
        let entry = file.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file() {
            let is_ts_file = entry.path().extension().unwrap_or_default() == "ts";
            if is_ts_file {
                files_to_process.push(entry.path())
            }
        } else if file_type.is_dir() {
            files_to_process.extend(get_files_paths(entry.path().to_str().unwrap().to_string()));
        }
    }
    files_to_process
}

fn main() {
    let start_time = Instant::now();
    let args = ActArgs::parse();
    let folder_path = args.folder_path;
    let files = get_files_paths(folder_path);
    for file_path in files {
        // TODO: bench single thread vs multi thread
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
    let duration = start_time.elapsed();
    let ms = duration.as_millis();
    println!("Act done in {}ms", ms)
}

#[cfg(test)]
mod tests {

    use super::process_file;
    use std::{fs, path::PathBuf, println};

    #[test]
    fn simple_function_test() {
        // TODO:
        // create tests folder if not exists
        fs::create_dir("tests").unwrap_or_else(|err| {
            println!("{:?}", err);
            panic!();
        });
        // something like that
        // ActArgs::parse_from(&["act", "-f", "tests"]);
        // create simple_function.ts file
        let file_path = PathBuf::from("tests/simple_function.ts");
        let file_data = r#"
        function test(a: string, b: number): string {
            return a + b;
        }"#;
        let expected_result_file_data = r#"
        function test(a: string, b: number): string {
            return a + b;
        }"#;
        fs::write(&file_path, file_data).unwrap_or_else(|err| {
            println!("{:?}", err);
            panic!();
        });
        process_file(file_path.clone()).unwrap_or(());
        // change file_path root folder path to out_folder_path
        let result = fs::read_to_string(&file_path).unwrap_or_else(|err| {
            println!("{:?}", err);
            panic!();
        });

        assert!(result == expected_result_file_data)
    }
}

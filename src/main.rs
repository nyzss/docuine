use clap::Parser;
use ignore::Walk;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to the directory to scan
    #[arg(short, long, default_value = "./")]
    path: String,

    /// output file
    #[arg(short, long, default_value = "./output.txt")]
    output: String,
}

fn is_lock_file(file_name: &str) -> bool {
    const LOCK_EXT: [&str; 2] = ["lock", "lock.json"];

    for ext in LOCK_EXT {
        if file_name.ends_with(ext) {
            return true;
        }
    }

    return false;
}

fn main() {
    let args = Args::parse();

    let dir_path = args.path;
    println!("Scanning path: {}", dir_path);

    let mut files_display = String::new();
    for result in Walk::new(dir_path) {
        match result {
            Ok(entry) => {
                let path = entry.path();
                let path_str = path.to_str().unwrap();

                let is_file = entry.file_type().unwrap().is_file();

                if is_file && !is_lock_file(path_str) {
                    let file_content = fs::read_to_string(entry.path());
                    match file_content {
                        Ok(content) => {
                            files_display.push_str(&format!("FILE_CONTENT: {}\n", content));
                        }
                        Err(e) => {
                            println!("Error reading file: {}", e);
                            files_display.push_str(&format!("FILE_CONTENT: {}\n", e));
                        }
                    }
                }

                let path = format!("{}\n", entry.path().to_str().unwrap());
                files_display.push_str(&path);
            }
            _ => {}
        }
    }

    println!("{}", files_display);

    fs::write(args.output, files_display).unwrap();
}

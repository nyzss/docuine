use clap::Parser;
use dotenv::dotenv;

use ignore::Walk;
use std::fs;

use rig::{completion::Prompt, providers::openai};

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

fn get_dir_structure(dir_path: String) -> String {
    let mut files_display = String::new();
    for result in Walk::new(dir_path) {
        match result {
            Ok(entry) => {
                let path = format!("{}\n", entry.path().to_str().unwrap_or("error_path"));
                files_display.push_str(&path);
            }
            _ => {}
        }
    }

    return files_display;
}

fn scan_files(dir_path: String) -> String {
    println!("Scanning path: {}", dir_path);

    let mut files_display = String::new();
    for result in Walk::new(dir_path) {
        match result {
            Ok(entry) => {
                let path = entry.path();
                let path_str = path.to_str().unwrap();

                let is_file = entry.file_type().unwrap().is_file();

                if is_file && !is_lock_file(path_str) {
                    let file_content = fs::read_to_string(path);
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
    return files_display;
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Args::parse();
    let openai_client = openai::Client::from_env();
    let gpt4 = openai_client
        .agent("gpt-4")
        .preamble(
            "You are a helpful assistant that can analyze the files and provide me a list of the files that are interesting to use to build a documentation for the project.",
        )
        .build();
    let path = args.path;

    let files_display = scan_files(path.clone());

    fs::write(args.output, files_display).unwrap();

    let response = gpt4
        .prompt(format!("The files: {}", get_dir_structure(path.clone())))
        .await
        .expect("Failed to prompt GPT-4");

    println!("GPT-4: {response}");
}

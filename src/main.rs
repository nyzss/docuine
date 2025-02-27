use ignore::Walk;
use std::fs;

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
    let mut files_display = String::new();
    for result in Walk::new("./") {
        match result {
            Ok(entry) => {
                let path = entry.path();
                let path_str = path.to_str().unwrap();

                let is_file = entry.file_type().unwrap().is_file();

                if is_file && !is_lock_file(path_str) {
                    let file_content = fs::read_to_string(entry.path()).unwrap();

                    println!("FILE_CONTENT: {}", file_content);
                }

                let path = format!("{}\n", entry.path().to_str().unwrap());
                files_display.push_str(&path);
            }
            _ => {}
        }
    }

    println!("{}", files_display);
}

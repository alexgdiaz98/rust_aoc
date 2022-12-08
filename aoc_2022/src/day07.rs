use core::panic;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

const MAX_SIZE: usize = 70000000;
const MIN_UNUSED: usize = 30000000;

/* Note: When representing paths as strings, we join the path segments with the "/" char.
This results in paths beginning with "//dir" rather than the OS norm, "/dir". */

#[allow(dead_code)]
pub fn day07(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    // Represents CWD path, where each string is a section of the path (e.g. //foo/bar => ["/", "foo", "bar"]).
    let mut path: Vec<&str> = vec![];
    // Maps a full directory path to the size of the directory (e.g. "//a/e" => 584 in description example.).
    let mut directory_sizes: HashMap<String, usize> = HashMap::new();
    for line in input.split('\n') {
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("$") => {
                // Incoming command
                match tokens.next() {
                    Some("cd") => match tokens.next() {
                        Some("/") => {
                            path = vec!["/"];
                        }
                        Some("..") => {
                            path.pop();
                        }
                        Some(dir) => path.push(dir),
                        None => panic!("Expected directory token. None encountered"),
                    },
                    Some("ls") => {}
                    Some(command) => panic!("Encountered unknown command: {}", command),
                    None => panic!("Expected command token. None encountered"),
                }
            }
            Some("dir") => {}
            Some(file_size_token) => {
                let file_size: usize = file_size_token
                    .parse()
                    .expect("Error parsing file size token");
                let file_name = tokens
                    .next()
                    .expect("Expected file_name token. None encountered");
                let mut file_path = path.join(&"/");
                file_path.push('/');
                file_path.push_str(file_name);
                let mut parents: Vec<&str> = path.clone();
                while parents.len() > 0 {
                    let parent_path = parents.join("/");
                    directory_sizes
                        .entry(String::from(parent_path))
                        .and_modify(|count| *count += file_size)
                        .or_insert(file_size);
                    parents.pop();
                }
            }
            _ => {}
        }
    }
    let mut p1: usize = 0;
    let mut p2: usize = usize::MAX;
    let space_needed: usize = MIN_UNUSED - (MAX_SIZE - directory_sizes.get("/").unwrap());
    for size in directory_sizes.values() {
        if *size <= 100000 {
            p1 += size;
        }
        if *size >= space_needed && *size < p2 {
            p2 = *size;
        }
    }
    (p1.to_string(), p2.to_string())
}

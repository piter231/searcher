use std::fs;
use std::env;
use std::sync::mpsc;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args[1].clone();
    let paths = fs::read_dir("../files/").unwrap();

    let (tx, rx) = mpsc::channel();

    for path in paths {
        let tx = tx.clone();
        let query = query.clone();
        let path = path.unwrap().path();

        thread::spawn(move || {
            if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        if content.contains(&query) {
                            let matching_lines: Vec<&str> = content.lines()
                                .filter(|line| line.contains(&query))
                                .collect();
                            let result = matching_lines.join("\n");
                            tx.send((path, result)).unwrap();
                        }
                    },
                    Err(_e) => (),
                }
            }
        });
    }

    drop(tx);

    for (path, content) in rx {
        println!("file:\n{}", path.display());
        println!("Content:\n{}", content);
    }
}

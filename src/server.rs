use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;

use crate::generate::generate;

pub fn serve(src_path: PathBuf, out_path: PathBuf) {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher
        .watch(&src_path.as_ref(), RecursiveMode::Recursive)
        .unwrap();

    for res in rx {
        match res {
            Ok(_) => match generate(&src_path, &out_path) {
                Ok(ok) => {
                    println!("Success: {}", ok);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

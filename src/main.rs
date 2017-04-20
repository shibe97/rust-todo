extern crate rustc_serialize;

use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json

struct Task {
    title: String,
    status: bool,
}

impl Task {
    fn new(title: &str) -> Task {
        Task {
            title: title.to_string(),
            status: false,
        }
    }
}

fn main() {
    let mut f = File::create("tasks.txt").unwrap();

    let task = Task::new("English");
    let task = json::encode(&task).unwrap();

    f.write_all(task).unwrap();
}

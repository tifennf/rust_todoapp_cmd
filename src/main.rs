use std::fs;
use std::process;
use std::path::Path;

const FILE_PATH: &str = "./todo_list.txt";

fn main() {
	if !(Path::new(FILE_PATH).exists()) {
		if let Err(err) = fs::File::create(FILE_PATH) {
			eprint!("Error when creating file {}: {}", FILE_PATH, err);
			process::exit(1);
		}
	}

	loop {
		println!("");
		todo_app::run(FILE_PATH);
	}

}

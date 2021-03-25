use std::fs;
use std::process;

const FILE_PATH: &str = "./todo_list.txt";

fn main() {
	loop {
		if let Err(err) = fs::File::create(FILE_PATH) {
			eprint!("Error when creating file: {}", err);
			process::exit(1);
		}
		
		println!("");
		todo_app::run(FILE_PATH);
	}

}

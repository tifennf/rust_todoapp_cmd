
use std::{fs, io, usize};
use std::process;
// use std::io::prelude::*;

//side effect fn area

pub fn run(file_path: &str) {
	let todo_list = fs::read_to_string(file_path).unwrap_or_else(| err | {
		eprintln!("Application error when reading file: {}", err);
		process::exit(1);
	});

	display_todo(&todo_list);
	display_order();

	let order = handle_user();

	match order {
		Order::Add(todo) => {
			let new_todo_list = add_todo(&todo, &todo_list);

			if let Err(err) = fs::write(file_path, new_todo_list.as_bytes()) {
				eprint!("Application error when writting file: {}", err);
				process::exit(1);
			}

		}
		Order::Del(line) => { 
			let new_todo_list = delete_todo(line, &&todo_list);

			if let Err(err) = fs::write(file_path, new_todo_list.as_bytes()) {
				eprint!("Application error when writting file: {}", err);
				process::exit(1);
			}
		}
		Order::Exit => process::exit(0),
	}

}

//ugly side effect fn
fn handle_user() -> Order {
	loop {
		let mut user_order = String::new();
		if let Err(_) = io::stdin().read_line(&mut user_order) {
			println!("Invalid order, try again.");
			continue;
		}

		let user_order: i32 = match user_order.trim().parse() {
			Ok(val) => val,
			Err(_) => {
				println!("You need to enter a number...");
				continue
			},
		};

		if user_order == 1 {

			let new_todo = loop {
				let mut todo = String::new();

				println!("Write ur todo: ");

				if let Err(err) = io::stdin().read_line(&mut todo) {
					println!("Invalid order, try again: {}", err);
					continue;
				}

				break todo;
			};

			break Order::Add(new_todo);

		} else if user_order == 2 {
			let line_to_delete = loop {

				println!("Todo to delete (number): ");

				let mut user_order = String::new();
				if let Err(_) = io::stdin().read_line(&mut user_order) {
					println!("Invalid order, try again.");
					continue;
				}
	
				let user_order: usize = match user_order.trim().parse() {
					Ok(val) => val,
					Err(_) => continue,
				};

				break user_order;
			};

			break Order::Del(line_to_delete);

		} else {
			break Order::Exit;
		}
	}
}

//pure fn area

fn add_todo(todo: &str, todo_list: &str) -> String {
	format!("{}{}", todo, todo_list)

}

fn delete_todo(line: usize, todo_list: &str) -> String {
	let mut clean_todo_list = String::new();

	for (todo_index, todo) in todo_list.lines().rev().enumerate() {
		if todo_index + 1 == line {
			continue;
		}

		clean_todo_list.push_str(&format!("{}\n", todo));
	}

	//needed to keep the correct display order after deleting
	let mut new_todo_list = String::new();
	for todo in clean_todo_list.lines().rev() {
		new_todo_list.push_str(&format!("{}\n", todo));
	}

	new_todo_list

}



fn display_order() {
	println!("\n========================================\n1. Add todo | 2. Delete todo | 3. Exit\n========================================");

}

fn display_todo(todo_list: &str) {
	for (todo_index, todo) in todo_list.lines().rev().enumerate(){
		println!("{}. {}", todo_index + 1, todo);
	}

}

// struct UserOrder {
// 	value: i32,
// }

// impl UserOrder {
// 	pub fn new<'a>(value: i32) -> UserOrder {
// 		if value < 1 || value > 3 {
// 			panic!("Incorrect order");
// 		}

// 		UserOrder { value }

// 	}

// 	pub fn value(&self) -> i32 {
// 		self.value

// 	}
// }

enum Order {
	Add(String),
	Del(usize),
	Exit,
}
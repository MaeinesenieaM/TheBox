pub mod corner;

pub fn select_item (choice: i32) {
	match choice {
		1 => {
		corner::start();
		},
		_ => {
		println! ("COULD NOT FIND ANYTHING!");
		}
	}
}
use gui::{Screen, Button, Draw};

struct SelectBox {
	width: u32,
	height: u32,
	options: Vec<String>,
}

impl Draw for SelectBox {
	fn draw(&self) {
		// do something here.
	}
}

fn main() {
	let screen = Screen {
		components: vec![
			Box::new(SelectBox {
				width: 200,
				height: 50,
				options: vec![
					String::from("A"),
					String::from("B"),
					String::from("C"),
				],
			}),
			Box::new(Button {
				width: 100,
				height: 50,
				label: String::from("OK"),
			})
		],
	};
	screen.run();
}
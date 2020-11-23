// #![windows_subsystem="windows"]
#![allow(unused_variables)]
#![allow(non_snake_case)]

extern crate sha1;

#[macro_use]
extern crate sciter;
use sciter::{ HELEMENT, Element, Value };
mod api;
use api::structs::UIEvents;

impl UIEvents {
	fn calc_sum(&mut self, a: i32, b: i32) -> i32 {
		a + b
    }
}

impl sciter::EventHandler for UIEvents {
	fn attached(&mut self, root: HELEMENT) {
		self.root = Some(Element::from(root));
	}

	dispatch_script_call! {
        fn calc_sum(i32, i32);
		fn User();
		fn Tax();
		fn Vote();
		fn Branch();
		fn Settings();
		fn Category();
		fn Employee();
		fn Measurement();
		fn SalaryScale();
		fn Penalty();
		fn Supplier();
		fn Customer();
		fn Product();
	}

	fn on_script_call(&mut self, root: HELEMENT, name: &str, argv: &[Value]) -> Option<Value> {
		let args = argv.iter().map(|x| format!("{:?}", &x)).collect::<Vec<String>>().join(", ");
		let handled = self.dispatch_script_call(root, name, argv);
		if handled.is_some() {
			return handled;
		}
		None
	}
}

fn main() {
	let login = "C:\\Users\\hsemi\\Desktop\\ME\\2020\\projects\\Supermarket\\ui\\main.html";
    // let resources = include_bytes!("all_users.rc");
    let handler = UIEvents { root: None };
    let mut frame = sciter::Window::new();
    // frame.archive_handler(resources).expect("Invalid archive");
    frame.event_handler(handler);
    frame.load_file(login);
    // frame.load_file("this://app/index.htm");
    frame.run_app();
}
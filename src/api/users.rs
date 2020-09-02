use super::structs::*;

use sciter::{ Value };

use super::modss::*;

impl UIEvents {	
	pub fn User(&mut self) -> Value {

		fn login(args: &[Value]) -> Value {
            Value::from("Login")
        }

        fn register(args: &[Value]) -> Value {
            Value::from("Register")
		}
		
		fn getUsers(args: &[Value]) -> Value {
			Value::from(methods::get_users())
		}

        // fn on_add(args: &[Value]) -> Value {
		// 	let ints = args.iter().map(|x| x.to_int().unwrap());
		// 	// let sum: i32 = ints.sum();	// error: issue #27739
		// 	let sum: i32 = ints.sum();
		// 	Value::from(sum)
		// }

		// fn on_sub(args: &[Value]) -> Value {
		// 	if args.len() != 2 || args.iter().any(|x| !x.is_int()) {
		// 		return Value::error("sub requires 2 integer arguments!");
		// 	}
		// 	let ints: Vec<_> = args.iter().map(|x| x.to_int().unwrap()).collect();
		// 	let (a,b) = (ints[0], ints[1]);
		// 	Value::from(a - b)
		// }

        let mut api = Value::new();

        api.set_item("login", login);
        api.set_item("register", register);
		api.set_item("get_users", getUsers);
		api
    }
    
}
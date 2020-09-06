use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn User(&mut self) -> Value {

		fn login(args: &[Value]) -> Value {
            Value::from("Login")
        }

        fn register(args: &[Value]) -> Value {
			
			let message = methods::save_user(&args[0]);

			let result = vmap!{
				"status" => true,
				"message" => "You're successful",
			};			
			Value::from(result)
		}
		
		fn getUsers(args: &[Value]) -> Value {
			Value::from(methods::get_users())
		}

        let mut api = Value::new();

        api.set_item("login", login);
        api.set_item("register", register);
		api.set_item("get_users", getUsers);
		api
    }
    
}
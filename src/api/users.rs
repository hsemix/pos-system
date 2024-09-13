use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn User(&mut self) -> Value {

		fn login(args: &[Value]) -> Value {
			
			let message = methods::login_user(&args[0]);

			if message.len() > 0 {
				let gotUser = serde_json::to_string(&message[0]).unwrap();

				let user: Value = gotUser.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "User data found",
					"data" => user,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "Wrong Username and or Password combination",
			})
			
			// Value::from(message)
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
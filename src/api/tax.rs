use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn Tax(&mut self) -> Value {

        fn save(args: &[Value]) -> String {
			let result;
			if checkTax() == true {
				result = db_methods_tax::update_tax(&args[0], Value::from(1));
			} else {
				result = db_methods_tax::save_tax(&args[0]);
			}
			
			result
		}
		
		fn getTax(args: &[Value]) -> Value {
			let result = db_methods_tax::get_tax(Value::from(1));

			if result.len() > 0 {
				let gotTax = serde_json::to_string(&result[0]).unwrap();

				let tax: Value = gotTax.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Tax data found",
					"data" => tax,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No tax saved yet",
			})
		}

		fn checkTax() -> bool {
			let result = db_methods_tax::get_tax(Value::from(1));

			if result.len() > 0 {
				return true;
			}
			false
		}

        let mut api = Value::new();

        api.set_item("save", save);
        api.set_item("getTax", getTax);
		api
    }
    
}
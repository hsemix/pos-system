use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn Customer(&mut self) -> Value {

        fn newCustomer(args: &[Value]) -> Value {
			let result = db_methods_customer::new_customer(&args[0]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}

		fn updateCustomer(args: &[Value]) -> Value {
			let result = db_methods_customer::update_customer(&args[0], &args[1]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}
		
		fn getCustomer(args: &[Value]) -> Value {
			let result = db_methods_customer::get_customer(&args[0]);

			if result.len() > 0 {
				let gotCustomer = serde_json::to_string(&result[0]).unwrap();

				let customer: Value = gotCustomer.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Customer data found",
					"data" => customer,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Customer with that ID",
			})
		}

		fn getCustomers(args: &[Value]) -> Value {
			let result = db_methods_customer::get_customers();

			if result.len() > 0 {
				let gotCustomers = serde_json::to_string(&result).unwrap();
				let customers: Value = gotCustomers.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Customers data found",
					"data" => customers,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Customers",
			})
		}

		fn deleteCustomer(args: &[Value]) -> String {
			let result = db_methods_customer::delete_customer(&args[0]);
			result
		}

        let mut api = Value::new();

        api.set_item("newCustomer", newCustomer);
		api.set_item("getCustomer", getCustomer);
		api.set_item("getCustomers", getCustomers);
		api.set_item("updateCustomer", updateCustomer);
		api.set_item("deleteCustomer", deleteCustomer);
		api
    }
}
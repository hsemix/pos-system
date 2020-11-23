use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn Employee(&mut self) -> Value {

        fn newEmployee(args: &[Value]) -> Value {
			let result = db_methods_employee::new_employee(&args[0]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}

		fn updateEmployee(args: &[Value]) -> Value {
			let result = db_methods_employee::update_employee(&args[0], &args[1]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}
		
		fn getEmployee(args: &[Value]) -> Value {
			let result = db_methods_employee::get_employee(&args[0]);

			if result.len() > 0 {
				let gotEmployee = serde_json::to_string(&result[0]).unwrap();

				let employee: Value = gotEmployee.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Employee data found",
					"data" => employee,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Employee with that ID",
			})
		}

		fn getEmployees(args: &[Value]) -> Value {
			let result = db_methods_employee::get_employees();

			if result.len() > 0 {
				let gotEmployees = serde_json::to_string(&result).unwrap();
				let employees: Value = gotEmployees.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Employees data found",
					"data" => employees,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Employees found",
			})
		}

		fn deleteEmployee(args: &[Value]) -> String {
			let result = db_methods_employee::delete_employee(&args[0]);
			result
		}

        let mut api = Value::new();

        api.set_item("newEmployee", newEmployee);
		api.set_item("getEmployee", getEmployee);
		api.set_item("getEmployees", getEmployees);
		api.set_item("updateEmployee", updateEmployee);
		api.set_item("deleteEmployee", deleteEmployee);
		api
    }
}
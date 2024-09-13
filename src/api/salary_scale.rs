use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn SalaryScale(&mut self) -> Value {

        fn newSalaryScale(args: &[Value]) -> Value {
			let result = db_methods_salary_scale::new_salary_scale(&args[0]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}

		fn updateSalaryScale(args: &[Value]) -> Value {
			let result = db_methods_salary_scale::update_salary_scale(&args[0], &args[1]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}
		
		fn getSalaryScale(args: &[Value]) -> Value {
			let result = db_methods_salary_scale::get_salary_scale(&args[0]);

			if result.len() > 0 {
				let gotSalaryScale = serde_json::to_string(&result[0]).unwrap();

				let salaryScale: Value = gotSalaryScale.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Salary Scale data found",
					"data" => salaryScale,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Salary Scale with that ID",
			})
		}

		fn getSalaryScales(args: &[Value]) -> Value {
			let result = db_methods_salary_scale::get_salary_scales();

			if result.len() > 0 {
				let gotSalaryScales = serde_json::to_string(&result).unwrap();
				let salaryScales: Value = gotSalaryScales.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Salary Scales data found",
					"data" => salaryScales,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Salary Scales found",
			})
		}

		fn deleteSalaryScale(args: &[Value]) -> String {
			let result = db_methods_salary_scale::delete_salary_scale(&args[0]);
			result
		}

        let mut api = Value::new();

        api.set_item("newSalaryScale", newSalaryScale);
		api.set_item("getSalaryScale", getSalaryScale);
		api.set_item("getSalaryScales", getSalaryScales);
		api.set_item("updateSalaryScale", updateSalaryScale);
		api.set_item("deleteSalaryScale", deleteSalaryScale);
		api
    }
}
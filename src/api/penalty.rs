use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn Penalty(&mut self) -> Value {

        fn newPenalty(args: &[Value]) -> Value {
			let result = db_methods_penalty::new_penalty(&args[0]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}

		fn updatePenalty(args: &[Value]) -> Value {
			let result = db_methods_penalty::update_penalty(&args[0], &args[1]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}
		
		fn getPenalty(args: &[Value]) -> Value {
			let result = db_methods_penalty::get_penalty(&args[0]);

			if result.len() > 0 {
				let gotPenalty = serde_json::to_string(&result[0]).unwrap();

				let penalty: Value = gotPenalty.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Penalty data found",
					"data" => penalty,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Penalty with that ID",
			})
		}

		fn getPenalties(args: &[Value]) -> Value {
			let result = db_methods_penalty::get_penalties();

			if result.len() > 0 {
				let gotPenalties = serde_json::to_string(&result).unwrap();
				let penalties: Value = gotPenalties.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Penalties data found",
					"data" => penalties,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Penalties found",
			})
		}

		fn deletePenalty(args: &[Value]) -> String {
			let result = db_methods_penalty::delete_penalty(&args[0]);
			result
		}

        let mut api = Value::new();

        api.set_item("newPenalty", newPenalty);
		api.set_item("getPenalty", getPenalty);
		api.set_item("getPenalties", getPenalties);
		api.set_item("updatePenalty", updatePenalty);
		api.set_item("deletePenalty", deletePenalty);
		api
    }
}
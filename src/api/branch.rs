use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn Branch(&mut self) -> Value {

        fn newBranch(args: &[Value]) -> Value {
			
			if db_methods_branch::get_main_branch().len() > 0 && &args[0]["is_main"].to_bool().unwrap() == &true {
				return 	Value::from(vmap!{
					"status" => false,
					"message" => "There can only be one main branch",
				});
			} else {
				let result = db_methods_branch::new_branch(&args[0]);
				return 	Value::from(vmap!{
					"status" => true,
					"message" => result,
				});
			}
		}

		fn updateBranch(args: &[Value]) -> Value {
			if db_methods_branch::get_main_branch().len() > 0 && &args[0]["is_main"].to_bool().unwrap() == &true && db_methods_branch::get_main_branch()[0].id.to_string() != args[1].to_string().trim_matches('"').to_string() {
				return 	Value::from(vmap!{
					"status" => false,
					"message" => "There can only be one main branch",
				});
			} else {
				let result = db_methods_branch::update_branch(&args[0], &args[1]);
				return 	Value::from(vmap!{
					"status" => true,
					"message" => result,
				});
			}
		}
		
		fn getBranch(args: &[Value]) -> Value {
			let result = db_methods_branch::get_branch(&args[0]);

			if result.len() > 0 {
				let gotBranch = serde_json::to_string(&result[0]).unwrap();

				let branch: Value = gotBranch.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Branch data found",
					"data" => branch,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Branch with that ID",
			})
		}

		fn getBranches(args: &[Value]) -> Value {
			let result = db_methods_branch::get_branches();

			if result.len() > 0 {
				let gotBranches = serde_json::to_string(&result).unwrap();
				let branches: Value = gotBranches.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Branches data found",
					"data" => branches,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Branches",
			})
		}

		fn getMainBranch(args: &[Value]) -> Value {
			let result = db_methods_branch::get_main_branch();

			if result.len() > 0 {
				let gotBranch = serde_json::to_string(&result[0]).unwrap();

				let branch: Value = gotBranch.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Main Branch data found",
					"data" => branch,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Branch with that ID",
			})
		}

		fn deleteBranch(args: &[Value]) -> String {
			let result = db_methods_branch::delete_branch(&args[0]);
			result
		}

        let mut api = Value::new();

        api.set_item("newBranch", newBranch);
		api.set_item("getBranch", getBranch);
		api.set_item("getBranches", getBranches);
		api.set_item("updateBranch", updateBranch);
		api.set_item("getMainBranch", getMainBranch);
		api.set_item("deleteBranch", deleteBranch);
		api
    }
}
use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn Category(&mut self) -> Value {

        fn newCategory(args: &[Value]) -> Value {
			let result = db_methods_category::new_category(&args[0]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}

		fn updateCategory(args: &[Value]) -> Value {
			let result = db_methods_category::update_category(&args[0], &args[1]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}
		
		fn getCategory(args: &[Value]) -> Value {
			let result = db_methods_category::get_category(&args[0]);

			if result.len() > 0 {
				let gotCategory = serde_json::to_string(&result[0]).unwrap();

				let category: Value = gotCategory.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Category data found",
					"data" => category,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Category with that ID",
			})
		}

		fn getCategories(args: &[Value]) -> Value {
			let result = db_methods_category::get_categories();

			if result.len() > 0 {
				let gotCategories = serde_json::to_string(&result).unwrap();
				let categories: Value = gotCategories.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Categories data found",
					"data" => categories,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Categories found",
			})
		}

		fn deleteCategory(args: &[Value]) -> String {
			let result = db_methods_category::delete_category(&args[0]);
			result
		}

        let mut api = Value::new();

        api.set_item("newCategory", newCategory);
		api.set_item("getCategory", getCategory);
		api.set_item("getCategories", getCategories);
		api.set_item("updateCategory", updateCategory);
		api.set_item("deleteCategory", deleteCategory);
		api
    }
}
use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn Supplier(&mut self) -> Value {

        fn newSupplier(args: &[Value]) -> Value {
			let result = db_methods_supplier::new_supplier(&args[0]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}

		fn updateSupplier(args: &[Value]) -> Value {
			let result = db_methods_supplier::update_supplier(&args[0], &args[1]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}
		
		fn getSupplier(args: &[Value]) -> Value {
			let result = db_methods_supplier::get_supplier(&args[0]);

			if result.len() > 0 {
				let gotSupplier = serde_json::to_string(&result[0]).unwrap();

				let supplier: Value = gotSupplier.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Supplier data found",
					"data" => supplier,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Supplier with that ID",
			})
		}

		fn getSuppliers(args: &[Value]) -> Value {
			let result = db_methods_supplier::get_suppliers();

			if result.len() > 0 {
				let gotSuppliers = serde_json::to_string(&result).unwrap();
				let supplliers: Value = gotSuppliers.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Suppliers data found",
					"data" => supplliers,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Suppliers",
			})
		}

		fn deleteSupplier(args: &[Value]) -> String {
			let result = db_methods_supplier::delete_supplier(&args[0]);
			result
		}

        let mut api = Value::new();

        api.set_item("newSupplier", newSupplier);
		api.set_item("getSupplier", getSupplier);
		api.set_item("getSuppliers", getSuppliers);
		api.set_item("updateSupplier", updateSupplier);
		api.set_item("deleteSupplier", deleteSupplier);
		api
    }
}
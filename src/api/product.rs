use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn Product(&mut self) -> Value {

        fn newProduct(args: &[Value]) -> Value {
			let result = db_methods_product::new_product(&args[0]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}

		fn updateProduct(args: &[Value]) -> Value {
			let result = db_methods_product::update_product(&args[0], &args[1]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}
		
		fn getProduct(args: &[Value]) -> Value {
			let result = db_methods_product::get_product(&args[0]);

			if result.len() > 0 {
				let gotProduct = serde_json::to_string(&result[0]).unwrap();

				let product: Value = gotProduct.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Product data found",
					"data" => product,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Product with that ID",
			})
		}

		fn getProducts(args: &[Value]) -> Value {
			let result = db_methods_product::get_products();

			if result.len() > 0 {
				let gotProducts = serde_json::to_string(&result).unwrap();
				let products: Value = gotProducts.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Products data found",
					"data" => products,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Products",
			})
		}

		fn deleteProduct(args: &[Value]) -> String {
			let result = db_methods_product::delete_product(&args[0]);
			result
		}

		fn getProducers(args: &[Value]) -> Value {
			let result = db_methods_product::get_producers();

			if result.len() > 0 {
				let gotProducers = serde_json::to_string(&result).unwrap();
				let producers: Value = gotProducers.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Producers data found",
					"data" => producers,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Producers",
			})
		}

        let mut api = Value::new();

        api.set_item("newProduct", newProduct);
		api.set_item("getProduct", getProduct);
		api.set_item("getProducts", getProducts);
		api.set_item("updateProduct", updateProduct);
		api.set_item("deleteProduct", deleteProduct);
		api.set_item("getProducers", getProducers);
		api
    }
}
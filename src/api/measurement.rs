use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn Measurement(&mut self) -> Value {

        fn newMeasurement(args: &[Value]) -> Value {
			let result = db_methods_measurement::new_measurement(&args[0]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}

		fn updateMeasurement(args: &[Value]) -> Value {
			let result = db_methods_measurement::update_measurement(&args[0], &args[1]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}
		
		fn getMeasurement(args: &[Value]) -> Value {
			let result = db_methods_measurement::get_measurement(&args[0]);

			if result.len() > 0 {
				let gotMeasurement = serde_json::to_string(&result[0]).unwrap();

				let measurement: Value = gotMeasurement.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Measurement data found",
					"data" => measurement,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Measurement with that ID",
			})
		}

		fn getMeasurements(args: &[Value]) -> Value {
			let result = db_methods_measurement::get_measurements();

			if result.len() > 0 {
				let gotMeasurements = serde_json::to_string(&result).unwrap();
				let measurements: Value = gotMeasurements.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Measurements data found",
					"data" => measurements,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Measurements found",
			})
		}

		fn deleteMeasurement(args: &[Value]) -> String {
			let result = db_methods_measurement::delete_measurement(&args[0]);
			result
		}

        let mut api = Value::new();

        api.set_item("newMeasurement", newMeasurement);
		api.set_item("getMeasurement", getMeasurement);
		api.set_item("getMeasurements", getMeasurements);
		api.set_item("updateMeasurement", updateMeasurement);
		api.set_item("deleteMeasurement", deleteMeasurement);
		api
    }
}
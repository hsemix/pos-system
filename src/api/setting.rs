use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn Settings(&mut self) -> Value {

        fn save(args: &[Value]) -> String {
			let result;
			if checkSetting() == true {
				result = db_methods_setting::update_setting(&args[0], Value::from(1));
			} else {
				result = db_methods_setting::save_setting(&args[0]);
			}
			
			result
		}
		
		fn getSetting(args: &[Value]) -> Value {
			let result = db_methods_setting::get_setting(Value::from(1));

			if result.len() > 0 {
				let gotSetting = serde_json::to_string(&result[0]).unwrap();

				let setting: Value = gotSetting.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Settings data found",
					"data" => setting,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No setting saved yet",
			})
		}

		fn checkSetting() -> bool {
			let result = db_methods_setting::get_setting(Value::from(1));

			if result.len() > 0 {
				return true;
			}
			false
		}

        let mut api = Value::new();

        api.set_item("addUpdateSettings", save);
        api.set_item("getSetting", getSetting);
		api
    }
    
}
use super::structs::*;

use sciter::{ Value };

use super::mods::*;

impl UIEvents {	
	pub fn Vote(&mut self) -> Value {

        fn newVote(args: &[Value]) -> Value {
			let result = db_methods_vote::new_vote(&args[0]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}

		fn updateVote(args: &[Value]) -> Value {
			let result = db_methods_vote::update_vote(&args[0], &args[1]);
			return 	Value::from(vmap!{
				"status" => true,
				"message" => result,
			});
		}
		
		fn getVote(args: &[Value]) -> Value {
			let result = db_methods_vote::get_vote(&args[0]);

			if result.len() > 0 {
				let gotVote = serde_json::to_string(&result[0]).unwrap();

				let vote: Value = gotVote.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Vote data found",
					"data" => vote,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Vote with that ID",
			})
		}

		fn getVotes(args: &[Value]) -> Value {
			let result = db_methods_vote::get_votes();

			if result.len() > 0 {
				let gotVotes = serde_json::to_string(&result).unwrap();
				let votes: Value = gotVotes.parse().unwrap();
				return Value::from(vmap!{
					"status" => true,
					"message" => "Votes data found",
					"data" => votes,
				});
			}

			Value::from(vmap!{
				"status" => false,
				"message" => "No Votes found",
			})
		}

		fn deleteVote(args: &[Value]) -> String {
			let result = db_methods_vote::delete_vote(&args[0]);
			result
		}

        let mut api = Value::new();

        api.set_item("newVote", newVote);
		api.set_item("getVote", getVote);
		api.set_item("getVotes", getVotes);
		api.set_item("updateVote", updateVote);
		api.set_item("deleteVote", deleteVote);
		api
    }
}
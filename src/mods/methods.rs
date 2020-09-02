use super::structs::*;
use super::common::*;

// use serde_json::{ Result, Value, json };
pub fn get_users() -> String {
    let all_persons: Vec<User> =
        pool().prep_exec("SELECT id, name from person", ())
 
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, name) = mysql::from_row(row);
 
                    User {
                        id,
                        name
                    }
                }).collect()
            }).unwrap(); // Unwrap `Vec<Person>`

    serde_json::to_string(&all_persons).unwrap()
}
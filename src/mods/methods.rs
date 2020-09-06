use super::structs::*;
use super::common::*;
use sciter::{ Value };
use sha1::{ Sha1 };



// use serde_json::{ Result, Value, json };
pub fn get_users() -> String {
    let all_users: Vec<User> =
        pool().prep_exec("SELECT `id`, `fullname`, `username`, `status` FROM `users` ORDER BY `created_at` DESC", ())
 
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    // type DBUser<'a> = (i32, String, String, bool);
                    // let p: DBUser<'static> = mysql::from_row(row);
                    // let (id, fullname, username, status) = p;
                    let (id, fullname, username, status) = mysql::from_row(row);//: (i32, String, String, bool);
                    // let status: bool = if status == 1 { true } else { false };
                    User {
                        id,
                        fullname,
                        username,
                        status
                    }
                }).collect()
            }).unwrap(); // Unwrap `Vec<User>`

    serde_json::to_string(&all_users).unwrap()
}

pub fn save_user(user: &Value) -> String {
    struct Users {
        username: String,
        fullname: String,
        password: String,
    }
    let userObject = Users { 
        username: user["username"].to_string(),
        fullname: user["fullname"].to_string(), //0726067149-lyker
        password: user["password"].to_string(),
    };
    let query = "INSERT INTO `users` (`username`, `fullname`, `password`) VALUES (?, ?, ?)";
    pool().prep_exec(query, (
        userObject.username.trim_matches('"'), 
        userObject.fullname.trim_matches('"'), 
        Sha1::from(userObject.password.trim_matches('"')).digest().to_string()
    )).unwrap();

    "user saved".to_string()
}
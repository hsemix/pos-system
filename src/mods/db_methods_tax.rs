use super::structs::*;
use super::common::*;
use sciter::{ Value };

pub fn save_tax(tax: &Value) -> String {
    
    let query = "INSERT INTO `taxes` (`value`) VALUES (?)";
    pool().prep_exec(query, (
        tax["taxVal"].to_string().trim_matches('"').to_string(), 
    )).unwrap();

    "Tax saved".to_string()
}

pub fn update_tax(tax: &Value, id: Value) -> String {
    
    let query = "UPDATE `taxes` SET `value`=? WHERE `id`=?";
    pool().prep_exec(query, (
        tax["taxVal"].to_string().trim_matches('"').to_string(), 
        id.to_string().trim_matches('"').to_string(),
    )).unwrap();

    "Tax saved".to_string()
}

// pub fn get_tax_nostruct(tax: Value) -> Vec<mysql::Row> {
    
//     let query = "SELECT * FROM `taxes` WHERE `id`=? ORDER BY `created_at` DESC LIMIT 1";
//     let param = "1".to_string();
//     let result: Vec<mysql::Row> = pool().prep_exec(query, (
//         tax.to_string().trim_matches('"').to_string(),
//     )).map(|result| {
//         result.map(|result| result.unwrap()).map(|row| row).collect()
//     }).unwrap();

//     result
// }


pub fn get_tax(id: Value) -> Vec<Tax> {
    
    let query = "SELECT `id`, `value` FROM `taxes` WHERE `id`=? ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<Tax> = pool().prep_exec(query, (
        id.to_string().trim_matches('"').to_string(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, value) = mysql::from_row(row);
            Tax {
                id,
                value
            }
        }).collect()
    }).unwrap(); // Unwrap

    result
}
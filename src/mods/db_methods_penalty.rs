use super::structs::*;
use super::common::*;
use sciter::{ Value };
use chrono::prelude::*;

/**
 * Create a new Penalty
 */
pub fn new_penalty(penalty: &Value) -> String {
    let query = "INSERT INTO `penalties` (`employee_id`, `amount`, `description`, `until`) VALUES (?, ?, ?, ?)";
    pool().prep_exec(query, (
        penalty["employee_id"].to_int(), 
        penalty["amount"].to_int(), 
        penalty["description"].as_string().unwrap(), 
        penalty["until"].to_string().trim_matches('"').to_string(), 
    )).unwrap();
    "Penalty saved".to_string()
}

/**
 * Update a given penalty, given an ID
 */
pub fn update_penalty(penalty: &Value, id: &Value) -> String {
    let updatedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `penalties` SET `employee_id`=?, `amount`=?, `description`=?, `until`=?, `updated_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        penalty["employee_id"].to_int(), 
        penalty["amount"].to_int(), 
        penalty["description"].as_string().unwrap(), 
        penalty["until"].to_string().trim_matches('"').to_string(),  
        updatedAt.to_string(),
        id.to_int(),
    )).unwrap();
    "Penalty updated".to_string()
}

/**
 * Get a single penalty, given and ID
 */
pub fn get_penalty(id: &Value) -> Vec<EmployeePenalty> {
    let query = "SELECT `id`, `employee_id`, `amount`, `description`, `until`, `employee_name` FROM `penalty_view` WHERE `id`=? AND `deleted_at` IS NULL ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<EmployeePenalty> = pool().prep_exec(query, (
        id.to_int(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, employee_id, amount, description, until, employee_name) = mysql::from_row(row);
            EmployeePenalty { id, employee_id, amount, description, until, employee_name }
        }).collect()
    }).unwrap();
    result
}

/**
 * Get all penalties
 */
pub fn get_penalties() -> Vec<EmployeePenalty> {
    let query = "SELECT `id`, `employee_id`, `amount`, `description`, `until`, `employee_name` FROM `penalty_view` WHERE `deleted_at` IS NULL ORDER BY `created_at` DESC";
    let result: Vec<EmployeePenalty> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, employee_id, amount, description, until, employee_name) = mysql::from_row(row);
            EmployeePenalty { id, employee_id, amount, description, until, employee_name }
        }).collect()
    }).unwrap();
    result
}

/**
 * Delete a Penalty
 */
pub fn delete_penalty(id: &Value) -> String {
    let deletedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `penalties` SET `deleted_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        deletedAt.to_string(),
        id.to_int(),
    )).unwrap();
    "Penalty deleted".to_string()
}
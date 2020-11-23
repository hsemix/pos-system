use super::structs::*;
use super::common::*;
use sciter::{ Value };
use chrono::prelude::*;

/**
 * Create a new Salary Scale
 */
pub fn new_salary_scale(salaryScale: &Value) -> String {
    let query = "INSERT INTO `salary_scales` (`name`, `amount`) VALUES (?, ?)";
    pool().prep_exec(query, (
        salaryScale["name"].as_string().unwrap(), 
        salaryScale["amount"].as_string().unwrap(), 
    )).unwrap();
    "Salary Scale saved".to_string()
}

/**
 * Update a given salary scale, given an ID
 */
pub fn update_salary_scale(salaryScale: &Value, id: &Value) -> String {
    let updatedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `salary_scales` SET `name`=?, `amount`=?, `updated_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        salaryScale["name"].as_string().unwrap(), 
        salaryScale["amount"].as_string().unwrap(), 
        updatedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Salary Scale updated".to_string()
}

/**
 * Get a single salary scale, given and ID
 */
pub fn get_salary_scale(id: &Value) -> Vec<SalaryScale> {
    let query = "SELECT `id`, `name`, `amount` FROM `salary_scales` WHERE `id`=? AND `deleted_at` IS NULL ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<SalaryScale> = pool().prep_exec(query, (
        id.as_string().unwrap(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, amount) = mysql::from_row(row);
            SalaryScale { id, name, amount }
        }).collect()
    }).unwrap();
    result
}

/**
 * Get all salary scales
 */
pub fn get_salary_scales() -> Vec<SalaryScale> {
    let query = "SELECT `id`, `name`, `amount` FROM `salary_scales` WHERE `deleted_at` IS NULL ORDER BY `created_at` DESC";
    let result: Vec<SalaryScale> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, amount) = mysql::from_row(row);
            SalaryScale { id, name, amount }
        }).collect()
    }).unwrap();
    result
}

/**
 * Delete a salary scale
 */
pub fn delete_salary_scale(id: &Value) -> String {
    let deletedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `salary_scales` SET `deleted_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        deletedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Salary Scale deleted".to_string()
}
use super::structs::*;
use super::common::*;
use sciter::{ Value };
use chrono::prelude::*;

/**
 * Create a new Employee
 */
pub fn new_employee(employee: &Value) -> String {
    let query = "INSERT INTO `employees` (`name`, `phone`, `email`, `address`, `salary_scale_id`, `is_active`, `is_system_paid`) VALUES (?, ?, ?, ?, ?, ?, ?)";
    pool().prep_exec(query, (
        employee["name"].as_string().unwrap(), 
        employee["phone"].as_string().unwrap(), 
        employee["email"].as_string().unwrap(), 
        employee["address"].as_string().unwrap(), 
        employee["salary_scale_id"].to_int(), 
        employee["is_active"].to_bool(), 
        employee["system_paid"].to_bool(), 
    )).unwrap();
    "Employee saved".to_string()
}

/**
 * Update a given employee, given an ID
 */
pub fn update_employee(employee: &Value, id: &Value) -> String {
    let updatedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `employees` SET `name`=?, `phone`=?, `email`=?, `address`=?, `salary_scale_id`=?, `is_active`=?, `is_system_paid`=?, `updated_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        employee["name"].as_string().unwrap(), 
        employee["phone"].as_string().unwrap(), 
        employee["email"].as_string().unwrap(), 
        employee["address"].as_string().unwrap(), 
        employee["salary_scale_id"].to_int(), 
        employee["is_active"].to_bool(), 
        employee["system_paid"].to_bool(), 
        updatedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Employee updated".to_string()
}

/**
 * Get a single employee, given and ID
 */
pub fn get_employee(id: &Value) -> Vec<Employee> {
    let query = "SELECT `id`, `name`, `phone`, `email`, `address`, `salary_scale_id`, `is_active`, `is_system_paid` FROM `employees` WHERE `id`=? AND `deleted_at` IS NULL ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<Employee> = pool().prep_exec(query, (
        id.as_string().unwrap(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, phone, email, address, salary_scale_id, is_active, is_system_paid) = mysql::from_row(row);
            Employee { id, name, phone, email, address, salary_scale_id, is_active, is_system_paid }
        }).collect()
    }).unwrap();
    result
}

/**
 * Get all employees
 */
pub fn get_employees() -> Vec<Employee> {
    let query = "SELECT `id`, `name`, `phone`, `email`, `address`, `salary_scale_id`, `is_active`, `is_system_paid` FROM `employees` WHERE `deleted_at` IS NULL ORDER BY `created_at` DESC";
    let result: Vec<Employee> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, phone, email, address, salary_scale_id, is_active, is_system_paid) = mysql::from_row(row);
            Employee { id, name, phone, email, address, salary_scale_id, is_active, is_system_paid }
        }).collect()
    }).unwrap();
    result
}

/**
 * Delete a employee
 */
pub fn delete_employee(id: &Value) -> String {
    let deletedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `employees` SET `deleted_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        deletedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Employee deleted".to_string()
}
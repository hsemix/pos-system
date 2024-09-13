use super::structs::*;
use super::common::*;
use sciter::{ Value };
use chrono::prelude::*;

/**
 * Create a new Customer
 */
pub fn new_customer(customer: &Value) -> String {
    let query = "INSERT INTO `customers` (`name`, `phone`, `email`, `code`, `location`, `account_number`, `is_active`) VALUES (?, ?, ?, ?, ?, ?, ?)";
    pool().prep_exec(query, (
        customer["customer_name"].as_string().unwrap(), 
        customer["customer_phone"].as_string().unwrap(), 
        customer["customer_email"].as_string().unwrap(), 
        customer["customer_code"].as_string().unwrap(), 
        customer["customer_location"].as_string().unwrap(), 
        customer["account_number"].as_string().unwrap(), 
        customer["is_active"].to_bool(),
    )).unwrap();
    "Customer saved".to_string()
}

/**
 * Update a given Customer, given an ID
 */
pub fn update_customer(customer: &Value, id: &Value) -> String {
    let updatedAt: DateTime<Utc> = Utc::now();
    let query = "UPDATE `customers` SET `name`=?, `phone`=?, `email`=?, `location`=?, `is_active`=?, `updated_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        customer["customer_name"].as_string().unwrap(), 
        customer["customer_phone"].as_string().unwrap(), 
        customer["customer_email"].as_string().unwrap(), 
        customer["customer_location"].as_string().unwrap(), 
        customer["is_active"].to_bool(),
        updatedAt.to_string(),
        id.as_string(),
    )).unwrap();
    String::from("Customer updated")
}

/**
 * Get a single Customer, given an ID
 */
pub fn get_customer(id: &Value) -> Vec<Customer> {
    let query = "SELECT `id`, `name`, `phone`, `email`, `code`, `location`, `account_number`, `is_active`, FROM `customers` WHERE `id`=? AND `deleted_at` IS NULL ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<Customer> = pool().prep_exec(query, (
        id.as_string().unwrap(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, phone, email, code, location, account_number, is_active) = mysql::from_row(row);
            Customer { id, name, phone, email, code, location, account_number, is_active }
        }).collect()
    }).unwrap();
    result
}

/**
 * Get all customers
 */
pub fn get_customers() -> Vec<Customer> {
    let query = "SELECT `id`, `name`, `phone`, `email`, `code`, `location`, `account_number`, `is_active` FROM `customers` WHERE `deleted_at` IS NULL ORDER BY `created_at` DESC";
    let result: Vec<Customer> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, phone, email, code, location, account_number, is_active) = mysql::from_row(row);
            Customer { id, name, phone, email, code, location, account_number, is_active }
        }).collect()
    }).unwrap();
    result
}

/**
 * Delete a Customer
 */
pub fn delete_customer(id: &Value) -> String {
    let deletedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `customers` SET `deleted_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        deletedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Customer deleted".to_string()
}
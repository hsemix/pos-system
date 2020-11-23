use super::structs::*;
use super::common::*;
use sciter::{ Value };
use chrono::prelude::*;

/**
 * Create a new Supplier
 */
pub fn new_supplier(supplier: &Value) -> String {
    let query = "INSERT INTO `suppliers` (`name`, `phone`, `email`, `code`, `location`, `account_number`, `is_active`, `is_preferred`) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
    pool().prep_exec(query, (
        supplier["supplier_name"].as_string().unwrap(), 
        supplier["supplier_phone"].as_string().unwrap(), 
        supplier["supplier_email"].as_string().unwrap(), 
        supplier["supplier_code"].as_string().unwrap(), 
        supplier["supplier_location"].as_string().unwrap(), 
        supplier["account_number"].as_string().unwrap(), 
        supplier["is_active"].to_bool(), 
        supplier["is_preferred"].to_bool(), 
    )).unwrap();
    "Supplier saved".to_string()
}

/**
 * Update a given Supplier, given an ID
 */
pub fn update_supplier(supplier: &Value, id: &Value) -> String {
    let updatedAt: DateTime<Utc> = Utc::now();
    let query = "UPDATE `suppliers` SET `name`=?, `phone`=?, `email`=?, `location`=?, `is_active`=?, `is_preferred`=?, `updated_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        supplier["supplier_name"].as_string().unwrap(), 
        supplier["supplier_phone"].as_string().unwrap(), 
        supplier["supplier_email"].as_string().unwrap(), 
        supplier["supplier_location"].as_string().unwrap(), 
        supplier["is_active"].to_bool(), 
        supplier["is_preferred"].to_bool(), 
        updatedAt.to_string(),
        id.as_string(),
    )).unwrap();
    String::from("Supplier updated")
}

/**
 * Get a single Supplier, given an ID
 */
pub fn get_supplier(id: &Value) -> Vec<Supplier> {
    let query = "SELECT `id`, `name`, `phone`, `email`, `code`, `location`, `account_number`, `is_active`, `is_preferred` FROM `suppliers` WHERE `id`=? AND `deleted_at` IS NULL ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<Supplier> = pool().prep_exec(query, (
        id.as_string().unwrap(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, phone, email, code, location, account_number, is_active, is_preferred) = mysql::from_row(row);
            Supplier { id, name, phone, email, code, location, account_number, is_active, is_preferred }
        }).collect()
    }).unwrap();
    result
}

/**
 * Get all suppliers
 */
pub fn get_suppliers() -> Vec<Supplier> {
    let query = "SELECT `id`, `name`, `phone`, `email`, `code`, `location`, `account_number`, `is_active`, `is_preferred` FROM `suppliers` WHERE `deleted_at` IS NULL ORDER BY `created_at` DESC";
    let result: Vec<Supplier> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, phone, email, code, location, account_number, is_active, is_preferred) = mysql::from_row(row);
            Supplier { id, name, phone, email, code, location, account_number, is_active, is_preferred }
        }).collect()
    }).unwrap();
    result
}

/**
 * Delete a Supplier
 */
pub fn delete_supplier(id: &Value) -> String {
    let deletedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `suppliers` SET `deleted_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        deletedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Supplier deleted".to_string()
}
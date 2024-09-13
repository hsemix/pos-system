use super::structs::*;
use super::common::*;
use sciter::{ Value };
use chrono::prelude::*;

/**
 * Create a new Product
 */
pub fn new_product(product: &Value) -> String {
    let query = "INSERT INTO `products` (`name`, `brand`, `category_id`, `producer`, `description`, `is_active`, `is_not_sold`, `is_service_item`, `is_purchased`) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";
    pool().prep_exec(query, (
        product["product_name"].as_string().unwrap(), 
        product["product_brand"].as_string().unwrap(), 
        product["category_id"].to_int(), 
        product["product_manufacturer"].as_string().unwrap(), 
        product["product_specification"].as_string().unwrap(), 
        product["is_active"].to_bool(),
        product["is_not_sold"].to_bool(),
        product["is_service_item"].to_bool(),
        product["is_purchased"].to_bool(),
    )).unwrap();
    "Product saved".to_string()
}

/**
 * Update a given Product, given an ID
 */
pub fn update_product(product: &Value, id: &Value) -> String {
    let updatedAt: DateTime<Utc> = Utc::now();
    let query = "UPDATE `products` SET `name`=?, `brand`=?, `category_id`=?, `producer`=?, `description`=?, `is_active`=?, `is_not_sold`=?, `is_service_item`=?, `is_purchased`=?, `updated_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        product["product_name"].as_string().unwrap(), 
        product["product_brand"].as_string().unwrap(), 
        product["category_id"].to_int(), 
        product["product_manufacturer"].as_string().unwrap(), 
        product["product_specification"].as_string().unwrap(), 
        product["is_active"].to_bool(),
        product["is_not_sold"].to_bool(),
        product["is_service_item"].to_bool(),
        product["is_purchased"].to_bool(),
        updatedAt.to_string(),
        id.as_string(),
    )).unwrap();
    String::from("Product updated")
}

/**
 * Get a single Product, given an ID
 */
pub fn get_product(id: &Value) -> Vec<Product> {
    let query = "SELECT `id`, `name`, `brand`, `category_id`, `producer`, `description`, `is_active`, `is_not_sold`, `is_service_item`, `is_purchased`, FROM `products` WHERE `id`=? AND `deleted_at` IS NULL ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<Product> = pool().prep_exec(query, (
        id.as_string().unwrap(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, brand, category_id, producer, description, is_active, is_not_sold, is_service_item, is_purchased) = mysql::from_row(row);
            Product { id, name, brand, category_id, producer, description, is_active, is_not_sold, is_service_item, is_purchased }
        }).collect()
    }).unwrap();
    result
}

/**
 * Get all products
 */
pub fn get_products() -> Vec<Product> {
    let query = "SELECT `id`, `name`, `brand`, `category_id`, `producer`, `description`, `is_active`, `is_not_sold`, `is_service_item`, `is_purchased` FROM `products` WHERE `deleted_at` IS NULL ORDER BY `created_at` DESC";
    let result: Vec<Product> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, brand, category_id, producer, description, is_active, is_not_sold, is_service_item, is_purchased) = mysql::from_row(row);
            Product { id, name, brand, category_id, producer, description, is_active, is_not_sold, is_service_item, is_purchased }
        }).collect()
    }).unwrap();
    result
}

/**
 * Delete a Product
 */
pub fn delete_product(id: &Value) -> String {
    let deletedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `products` SET `deleted_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        deletedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Product deleted".to_string()
}

/**
 * Load product (recently added) manufacturers
 */
pub fn get_producers() -> Vec<Producer> {
    let deletedAt: DateTime<Local> = Local::now();
    let query = "SELECT `producer` AS `name` FROM `products` ORDER BY `created_at` DESC";
    let result: Vec<Producer> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let name = mysql::from_row(row);
            Producer { name }
        }).collect()
    }).unwrap();
    result
}
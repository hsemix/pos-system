use super::structs::*;
use super::common::*;
use sciter::{ Value };
use chrono::prelude::*;

/**
 * Create a new Category
 */
pub fn new_category(category: &Value) -> String {
    let query = "INSERT INTO `categories` (`name`) VALUES (?)";
    pool().prep_exec(query, (
        category["name"].as_string().unwrap(), 
    )).unwrap();
    "Category saved".to_string()
}

/**
 * Update a given category, given an ID
 */
pub fn update_category(category: &Value, id: &Value) -> String {
    let updatedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `categories` SET `name`=?, `updated_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        category["name"].as_string().unwrap(), 
        updatedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Category updated".to_string()
}

/**
 * Get a single category, given and ID
 */
pub fn get_category(id: &Value) -> Vec<Category> {
    let query = "SELECT `id`, `name` FROM `categories` WHERE `id`=? AND `deleted_at` IS NULL ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<Category> = pool().prep_exec(query, (
        id.as_string().unwrap(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name) = mysql::from_row(row);
            Category { id, name }
        }).collect()
    }).unwrap();
    result
}

/**
 * Get all categories
 */
pub fn get_categories() -> Vec<Category> {
    let query = "SELECT `id`, `name` FROM `categories` WHERE `deleted_at` IS NULL ORDER BY `created_at` DESC";
    let result: Vec<Category> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name) = mysql::from_row(row);
            Category { id, name }
        }).collect()
    }).unwrap();
    result
}

/**
 * Delete a category
 */
pub fn delete_category(id: &Value) -> String {
    let deletedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `categories` SET `deleted_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        deletedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Category deleted".to_string()
}
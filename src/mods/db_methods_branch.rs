use super::structs::*;
use super::common::*;
use sciter::{ Value };
use chrono::prelude::*;

/**
 * Create a new Branch
 */
pub fn new_branch(branch: &Value) -> String {
    let query = "INSERT INTO `branches` (`name`, `phone`, `location`, `is_main`, `pobox`, `website`, `email`) VALUES (?, ?, ?, ?, ?, ?, ?)";
    pool().prep_exec(query, (
        branch["name"].as_string().unwrap(), 
        branch["phone"].as_string().unwrap(), 
        branch["location"].as_string().unwrap(), 
        branch["is_main"].to_bool(), 
        branch["pobox"].as_string().unwrap(), 
        branch["website"].as_string().unwrap(), 
        branch["email"].as_string().unwrap(), 
    )).unwrap();
    // .unwrap_or_else(|error| {
    //     panic!("An error occurred: {:?}", error);
    // });
    "Branch saved".to_string()
}

/**
 * Update a given branch, given an ID
 */
pub fn update_branch(branch: &Value, id: &Value) -> String {
    let updatedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `branches` SET `name`=?, `phone`=?, `location`=?, `is_main`=?, `pobox`=?, `website`=?, `email`=?, `updated_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        branch["name"].as_string().unwrap(), 
        branch["phone"].as_string().unwrap(), 
        branch["location"].as_string().unwrap(), 
        branch["is_main"].to_bool(), 
        branch["pobox"].as_string().unwrap(), 
        branch["website"].as_string().unwrap(), 
        branch["email"].as_string().unwrap(), 
        updatedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Branch saved".to_string()
}

/**
 * Get a single branch, given and ID
 */
pub fn get_branch(id: &Value) -> Vec<Branch> {
    let query = "SELECT `id`, `name`, `phone`, `location`, `is_main`, `pobox`, `website`, `email` FROM `branches` WHERE `id`=? AND `deleted_at` IS NULL ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<Branch> = pool().prep_exec(query, (
        id.as_string().unwrap(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, phone, location, is_main, pobox, website, email) = mysql::from_row(row);
            Branch { id, name, phone, location, is_main, pobox, website, email }
        }).collect()
    }).unwrap();
    result
}

/**
 * Get all branches
 */
pub fn get_branches() -> Vec<Branch> {
    let query = "SELECT `id`, `name`, `phone`, `location`, `is_main`, `pobox`, `website`, `email` FROM `branches` WHERE `deleted_at` IS NULL ORDER BY `created_at` DESC";
    let result: Vec<Branch> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, phone, location, is_main, pobox, website, email) = mysql::from_row(row);
            Branch { id, name, phone, location, is_main, pobox, website, email }
        }).collect()
    }).unwrap();
    result
}

/**
 * Get main branch
 */
pub fn get_main_branch() -> Vec<Branch> {
    let query = "SELECT `id`, `name`, `phone`, `location`, `is_main`, `pobox`, `website`, `email` FROM `branches` WHERE `is_main`='1' AND `deleted_at` IS NULL ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<Branch> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, phone, location, is_main, pobox, website, email) = mysql::from_row(row);
            Branch { id, name, phone, location, is_main, pobox, website, email }
        }).collect()
    }).unwrap();
    result
}

/**
 * Delete a branch
 */
pub fn delete_branch(id: &Value) -> String {
    let deletedAt: DateTime<Utc> = Utc::now();
    let query = "UPDATE `branches` SET `deleted_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        deletedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Branch deleted".to_string()
}
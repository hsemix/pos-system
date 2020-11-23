use super::structs::*;
use super::common::*;
use sciter::{ Value };
use chrono::prelude::*;

/**
 * Create a new Vote
 */
pub fn new_vote(vote: &Value) -> String {
    let query = "INSERT INTO `votes` (`name`) VALUES (?)";
    pool().prep_exec(query, (
        vote["name"].as_string().unwrap(), 
    )).unwrap();
    "Vote saved".to_string()
}

/**
 * Update a given vote, given an ID
 */
pub fn update_vote(vote: &Value, id: &Value) -> String {
    let updatedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `votes` SET `name`=?, `updated_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        vote["name"].as_string().unwrap(), 
        updatedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Vote updated".to_string()
}

/**
 * Get a single vote, given and ID
 */
pub fn get_vote(id: &Value) -> Vec<Vote> {
    let query = "SELECT `id`, `name` FROM `votes` WHERE `id`=? AND `deleted_at` IS NULL ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<Vote> = pool().prep_exec(query, (
        id.as_string().unwrap(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name) = mysql::from_row(row);
            Vote { id, name }
        }).collect()
    }).unwrap();
    result
}

/**
 * Get all vote
 */
pub fn get_votes() -> Vec<Vote> {
    let query = "SELECT `id`, `name` FROM `votes` WHERE `deleted_at` IS NULL ORDER BY `created_at` DESC";
    let result: Vec<Vote> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name) = mysql::from_row(row);
            Vote { id, name }
        }).collect()
    }).unwrap();
    result
}

/**
 * Delete a vote
 */
pub fn delete_vote(id: &Value) -> String {
    let deletedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `votes` SET `deleted_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        deletedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Vote deleted".to_string()
}
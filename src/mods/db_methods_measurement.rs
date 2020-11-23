use super::structs::*;
use super::common::*;
use sciter::{ Value };
use chrono::prelude::*;

/**
 * Create a new Measurement
 */
pub fn new_measurement(measurement: &Value) -> String {
    let query = "INSERT INTO `measurements` (`name`) VALUES (?)";
    pool().prep_exec(query, (
        measurement["name"].as_string().unwrap(), 
    )).unwrap();
    "Measurement saved".to_string()
}

/**
 * Update a given measurement, given an ID
 */
pub fn update_measurement(measurement: &Value, id: &Value) -> String {
    let updatedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `measurements` SET `name`=?, `updated_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        measurement["name"].as_string().unwrap(), 
        updatedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Measurement updated".to_string()
}

/**
 * Get a single measurement, given and ID
 */
pub fn get_measurement(id: &Value) -> Vec<Measurement> {
    let query = "SELECT `id`, `name` FROM `measurements` WHERE `id`=? AND `deleted_at` IS NULL ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<Measurement> = pool().prep_exec(query, (
        id.as_string().unwrap(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name) = mysql::from_row(row);
            Measurement { id, name }
        }).collect()
    }).unwrap();
    result
}

/**
 * Get all measurements
 */
pub fn get_measurements() -> Vec<Measurement> {
    let query = "SELECT `id`, `name` FROM `measurements` WHERE `deleted_at` IS NULL ORDER BY `created_at` DESC";
    let result: Vec<Measurement> = pool().prep_exec(query, ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name) = mysql::from_row(row);
            Measurement { id, name }
        }).collect()
    }).unwrap();
    result
}

/**
 * Delete a measurement
 */
pub fn delete_measurement(id: &Value) -> String {
    let deletedAt: DateTime<Local> = Local::now();
    let query = "UPDATE `measurements` SET `deleted_at`=? WHERE `id`=?";
    pool().prep_exec(query, (
        deletedAt.to_string(),
        id.as_string().unwrap(),
    )).unwrap();
    "Measurement deleted".to_string()
}
use serde::{Deserialize, Serialize};

use chrono::prelude::*;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub status: bool
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Tax {
    pub id: i32,
    pub value: i32
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Branch {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub location: String,
    pub is_main: bool,
    pub pobox: String,
    pub website: String,
    pub email: String
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Setting {
    pub id: i32,
    pub include_vat: bool,
    pub include_vat_cents: bool,
    pub include_vat_on_burgain: bool,
    pub price_decimals: i32,
    pub quantity_decimals: i32,
    pub currency_symbol: String,
    pub symbol_position: String
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Measurement {
    pub id: i32,
    pub name: String
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Vote {
    pub id: i32,
    pub name: String
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SalaryScale {
    pub id: i32,
    pub name: String,
    pub amount: i32
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub address: String,
    pub salary_scale_id: i32,
    pub is_active: bool,
    pub is_system_paid: bool,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct EmployeePenalty {
    pub id: i32,
    pub employee_id: i32,
    pub amount: i32,
    pub description: String,
    pub until: NaiveDate,
    pub employee_name: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Supplier {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub code: String,
    pub location: String,
    pub account_number: i32,
    pub is_active: bool,
    pub is_preferred: bool,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub code: String,
    pub location: String,
    pub account_number: i32,
    pub is_active: bool,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Product {
    pub id: i32, 
    pub name: String, 
    pub brand: String, 
    pub category_id: i32, 
    pub producer: String, 
    pub description: String, 
    pub is_active: bool, 
    pub is_not_sold: bool, 
    pub is_service_item: bool, 
    pub is_purchased: bool,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Producer {
    pub name: String
}
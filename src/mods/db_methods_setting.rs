use super::structs::*;
use super::common::*;
use sciter::{ Value };

pub fn save_setting(setting: &Value) -> String {
    
    let query = "INSERT INTO `settings` (`price_decimals`, `quantity_decimals`, `currency_symbol`, `symbol_position`) VALUES (?, ?, ?, ?)";
    pool().prep_exec(query, (
        setting["price_decimal"].to_string().trim_matches('"').to_string(), 
        setting["quantity_decimal"].to_string().trim_matches('"').to_string(), 
        setting["currency_symbol"].to_string().trim_matches('"').to_string(), 
        setting["symbol_position"].to_string().trim_matches('"').to_string(), 
    )).unwrap();

    "Currency settings saved".to_string()
}

pub fn update_setting(setting: &Value, id: Value) -> String {
    
    let query = "UPDATE `settings` SET `price_decimals`=?, `quantity_decimals`=?, `currency_symbol`=?, `symbol_position`=? WHERE `id`=?";
    pool().prep_exec(query, (
        setting["price_decimal"].to_string().trim_matches('"').to_string(), 
        setting["quantity_decimal"].to_string().trim_matches('"').to_string(), 
        setting["currency_symbol"].to_string().trim_matches('"').to_string(), 
        setting["symbol_position"].to_string().trim_matches('"').to_string(), 
        id.to_string().trim_matches('"').to_string(),
    )).unwrap();

    "Currency settings saved".to_string()
}

pub fn get_setting(id: Value) -> Vec<Setting> {
    
    let query = "SELECT `id`, `include_vat`, `include_vat_cents`, `include_vat_on_burgain`, `price_decimals`, `quantity_decimals`, `currency_symbol`, `symbol_position` FROM `settings` WHERE `id`=? ORDER BY `created_at` DESC LIMIT 1";
    let result: Vec<Setting> = pool().prep_exec(query, (
        id.to_string().trim_matches('"').to_string(),
    )).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, include_vat, include_vat_cents, include_vat_on_burgain, price_decimals, quantity_decimals, currency_symbol, symbol_position) = mysql::from_row(row);
            Setting { id, include_vat, include_vat_cents, include_vat_on_burgain, price_decimals, quantity_decimals, currency_symbol, symbol_position }
        }).collect()
    }).unwrap();

    result
}
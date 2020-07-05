use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    /*this is the short way, but you can just match the result of the parse*/;

    Ok(item_quantity.parse::<i32>()? * cost_per_item + processing_fee)
}

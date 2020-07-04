use std::cmp::Ordering;

pub fn calculateprice(amount: i32) -> i32 {
    if amount>40 {
        amount
    }else {
        amount * 2
    }
}

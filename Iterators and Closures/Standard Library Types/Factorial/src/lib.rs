pub fn factorial(num: u64) -> u64 {
    if num==0 {
        1
    }else {
        (1..=num).collect::<Vec<_>>().iter().product()
    }
}

pub fn factorial(num: u64) -> u64 {
    (2..=num).collect::<Vec<_>>().iter().product()
}

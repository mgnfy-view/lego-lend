pub fn is_zero(x: &u64) -> bool {
    *x == 0
}

pub fn exactly_one_zero(x: &u64, y: &u64) -> bool {
    is_zero(x) ^ is_zero(y)
}

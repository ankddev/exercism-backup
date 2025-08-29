/// Get square of first n numbers' sum
pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).sum::<u32>().pow(2)
}

/// Get sum of first n numbers' squares
pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|num| num.pow(2)).sum()
}

/// Get difference between square of first n numbers' sum and sum of first n numbers' squares
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

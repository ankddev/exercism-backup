/// Return true if given number is Armstrong number
pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    num_string.chars().map(|d| d.to_digit(10).unwrap()).map(|num| num.pow(num_string.len() as u32)).sum::<u32>() == num
}

pub fn is_valid(code: &str) -> bool {
    let code: Vec<u32> = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
        .unwrap_or_default();
    if code.len() <= 1 {
        return false;
    }
    let sum: u32 = code
        .iter()
        .rev()
        .enumerate()
        .map(|(index, &num)| {
            if index % 2 == 1 {
                let doubled = num * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                num
            }
        })
        .sum();
    sum % 10 == 0
}

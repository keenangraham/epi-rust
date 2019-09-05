fn is_even(n: u32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    false
}


fn even_odd(numbers: &mut[u32]) -> &mut[u32] {
    let mut left: usize = 0;
    let mut right: usize = numbers.len() - 1;
    while left < right {
        if is_even(numbers[left]) {
            left += 1;
        } else {
            numbers.swap(left, right);
            right -= 1;
        }
    }
    numbers
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_is_even() {
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
    }

    
    #[test]
    fn test_even_odd() {
        assert_eq!(even_odd(&mut[1, 2, 3]), [2, 3, 1]);
        assert_eq!(
            even_odd(&mut[0, 0, 1, 2, 2, 3, 4, 5, 6, 7, 7, 8, 15, 24]),
            [0, 0, 24, 2, 2, 8, 4, 6, 7, 7, 5, 15, 3, 1]
        );
    }
}

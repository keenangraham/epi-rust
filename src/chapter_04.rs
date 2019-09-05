fn count_bits(mut x: u32) -> u32 {
    let mut num_bits = 0;
    while x != 0 {
        num_bits += x & 1;
        x >>= 1;
    }
    num_bits
}


fn count_bits_fun(x: u32) -> u32 {
    let binary_x = format!("{:b}", x);
    binary_x.chars().map(
        |bit| {
            bit.to_digit(10).unwrap()
        }
    ).sum::<u32>()
}


#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(1), 1);
        assert_eq!(count_bits(3), 2);
        assert_eq!(count_bits(8), 1);
        assert_eq!(count_bits(10013), 8);
        assert_eq!(count_bits(4294967292), 30);        
        assert_eq!(count_bits(std::u32::MAX), 32);
    }

    
    #[test]
    fn test_count_bits_fun() {
        assert_eq!(count_bits_fun(0), 0);
        assert_eq!(count_bits_fun(1), 1);
        assert_eq!(count_bits_fun(3), 2);
        assert_eq!(count_bits_fun(8), 1);
        assert_eq!(count_bits_fun(10013), 8);
        assert_eq!(count_bits_fun(4294967292), 30);        
        assert_eq!(count_bits_fun(std::u32::MAX), 32);
    }
}

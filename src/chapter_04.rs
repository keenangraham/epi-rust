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


fn clear_lowest_bit(x: u32) -> u32 {
    x & (x - 1)
}


fn extract_lowest_bit(x: u32) -> u32 {
    x & !(x - 1)
}


fn bits_are_equal(x: &u32, &first_position: &u32, second_position: &u32) -> bool {
    if (x >> first_position) & 1 == (x >> second_position) & 1 {
        return true;
    }
    false
}


fn swap_bits(mut x: u32, first_position: u32, second_position: u32) -> u32 {
    if !bits_are_equal(&x, &first_position, &second_position) {
        let bit_mask = (1 << first_position) | (1 << second_position);
        x ^= bit_mask;
    }
    x
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


    #[test]
    fn test_clear_lowest_bit() {
        assert_eq!(clear_lowest_bit(3), 2);
        assert_eq!(clear_lowest_bit(7), 6);
        assert_eq!(clear_lowest_bit(8), 0);
    }


    #[test]
    fn test_extract_lowest_bit() {
        assert_eq!(extract_lowest_bit(3), 1);
        assert_eq!(extract_lowest_bit(4), 4);
        assert_eq!(extract_lowest_bit(7), 1);
        assert_eq!(extract_lowest_bit(8), 8);
    }


    #[test]
    fn test_bits_are_equal() {
        assert_eq!(bits_are_equal(&3, &0, &0), true);
        assert_eq!(bits_are_equal(&3, &0, &1), true);
        assert_eq!(bits_are_equal(&4, &0, &1), true);
        assert_eq!(bits_are_equal(&4, &0, &2), false);
    }


    #[test]
    fn test_swap_bits() {
        assert_eq!(swap_bits(4, 0, 2), 1);
        assert_eq!(swap_bits(21, 2, 3), 25);
        assert_eq!(swap_bits(21, 3, 2), 25);
        assert_eq!(swap_bits(21, 3, 3), 21);
        assert_eq!(swap_bits(55, 0, 3), 62);        
    }
}

pub fn count_ones(value: u64) -> u8 {
    let mut count = 0;
    let mut new_value = value;
    for _ in 0..64 {
        if new_value & 1 == 1 {
            count += 1;
        }
        new_value = new_value >> 1;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn count_ones_empty() {
        assert_eq!(count_ones(0), 0);
    }   
    
    #[test]
    fn count_ones_one() {
        assert_eq!(count_ones(1), 1);
    }   
    
    #[test]
    fn count_ones_two() {
        assert_eq!(count_ones(2), 1);
    }   
    
    #[test]
    fn count_ones_three() {
        assert_eq!(count_ones(3), 2);
    }   
}
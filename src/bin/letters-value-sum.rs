fn main() {
    let letters = std::env::args().nth(1).expect("letters expected");
    println!("values sum: {}", lettersum(&letters));
}

fn lettersum(s: &str) -> i32 {
    let a = 'a';
    let z = 'z';

    s.chars()
        .filter(|c| c.ge(&a) && c.le(&z))
        .map(|c| c as i32 - a as i32 + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lettersum_a() -> Result<(), String> {
        let letters = "a";
        assert_eq!(lettersum(letters), 1);
        Ok(())
    }
    #[test]
    fn test_lettersum_z() -> Result<(), String> {
        let letters = "z";
        assert_eq!(lettersum(letters), 26);
        Ok(())
    }
    #[test]
    fn test_lettersum_az() -> Result<(), String> {
        let letters = "az";
        assert_eq!(lettersum(letters), 27);
        Ok(())
    }
    #[test]
    fn test_lettersum_empty() -> Result<(), String> {
        let letters = "";
        assert_eq!(lettersum(letters), 0);
        Ok(())
    }
    #[test]
    fn test_lettersum_nonaz_are_ignored() -> Result<(), String> {
        let letters = "a+B z";
        assert_eq!(lettersum(letters), 27);
        Ok(())
    }
}
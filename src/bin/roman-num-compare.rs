use itertools::{EitherOrBoth::*, Itertools};

fn main() {
    let r1 = std::env::args().nth(1).expect("r1 expected");
    let r2 = std::env::args().nth(2).expect("r2 expected");

    let smaller = if roman_num_smaller(&r1, &r2) {
        "smaller"
    } else {
        "not smaller"
    };
    println!("{} is {} than {}", r1, smaller, r2)
}

fn roman_num_smaller(r1: &str, r2: &str) -> bool {
    for pair in r1.chars().zip_longest(r2.chars()) {
        match pair {
            Both(l, r) => {
                let lo = roman_order(l);
                let ro = roman_order(r);
                if lo < ro {
                    return true;
                } else if lo > ro {
                    return false;
                }
            }
            Left(_l) => return false,
            Right(_r) => return true,
        }
    }
    false
}

fn roman_order(c: char) -> i16 {
    match c {
        'M' => 7,
        'D' => 6,
        'C' => 5,
        'L' => 4,
        'X' => 3,
        'V' => 2,
        'I' => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_order() -> Result<(), String> {
        assert_eq!(roman_order('M'), 7);
        assert_eq!(roman_order('D'), 6);
        assert_eq!(roman_order('C'), 5);
        assert_eq!(roman_order('L'), 4);
        assert_eq!(roman_order('X'), 3);
        assert_eq!(roman_order('V'), 2);
        assert_eq!(roman_order('I'), 1);
        assert_eq!(roman_order('+'), 0);
        Ok(())
    }

    #[test]
    fn test_roman_num_smaller() -> Result<(), String> {
        assert_eq!(roman_num_smaller("MM", "MM"), false);
        assert_eq!(roman_num_smaller("MM", "MD"), false);
        assert_eq!(roman_num_smaller("MD", "MM"), true);
        assert_eq!(roman_num_smaller("I", "II"), true);
        assert_eq!(roman_num_smaller("II", "I"), false);
        assert_eq!(roman_num_smaller("V", "IIII"), false);
        assert_eq!(roman_num_smaller("MDCLXV", "MDCLXVI"), true);
        assert_eq!(roman_num_smaller("MM", "MDCCCCLXXXXVIIII"), false);

        Ok(())
    }
}

fn main() {
    let bus_s = std::env::args().nth(1).expect("bus expected");
    let passenger_s = std::env::args().nth(2).expect("passenger expected");

    let bus = bus_s.parse::<i32>().unwrap();
    let passenger = passenger_s.parse::<i32>().unwrap();

    println!(
        "Bus:{} Passenger:{} Room:{}",
        bus,
        passenger,
        room_number(bus, passenger)
    );
}

fn room_number(b: i32, p: i32) -> i32 {
    let mut rn = 1 + b * (b - 1) / 2;
    if p > 1 {
        rn += (p - 1) * b + (p - 1) * p / 2;
    }
    return rn;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_passenger() -> Result<(), String> {
        assert_eq!(room_number(1, 1), 1);
        assert_eq!(room_number(2, 1), 2);
        assert_eq!(room_number(3, 1), 4);
        assert_eq!(room_number(4, 1), 7);
        assert_eq!(room_number(5, 1), 11);
        assert_eq!(room_number(6, 1), 16);
        Ok(())
    }

    #[test]
    fn test_random_passenger() -> Result<(), String> {
        assert_eq!(room_number(1, 3), 6);
        assert_eq!(room_number(2, 2), 5);
        assert_eq!(room_number(3, 4), 19);
        assert_eq!(room_number(4, 2), 12);
        assert_eq!(room_number(5, 2), 17);
        Ok(())
    }
}

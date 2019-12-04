fn main() {
    let modules = vec![
        112908, 61769, 65967, 51494, 99689, 114098, 135346, 59561, 147324, 50465, 117491, 77845,
        91959, 59847, 84013, 85763, 62121, 58965, 89809, 97870, 77696, 70218, 118404, 83505, 141729,
        61534, 101073, 131358, 76104, 120836, 109789, 96510, 65406, 117906, 74921, 95412, 99875, 134298,
        105235, 82802, 103392, 81906, 133786, 140681, 109004, 148434, 92333, 83848, 98297, 95728,
        138202, 79312, 55633, 138461, 50293, 141922, 140303, 66542, 50054, 99076, 143201, 66702, 133583,
        70308, 146824, 95606, 63054, 129272, 133051, 58626, 119896, 66265, 99925, 131752, 74669, 148387,
        132124, 107188, 55535, 145004, 78122, 50885, 70325, 100859, 86484, 88795, 148164, 64473, 143089,
        121023, 52904, 120927, 87164, 133709, 89427, 105350, 106378, 98492, 78394, 145200
    ];

    let mut fuel = modules.iter().fold(0, |acc, mass| acc + required_fuel(*mass));
    println!("part 1 - Total fuel = {}", fuel);

    fuel = modules.iter().fold(0, |acc, mass| acc + total_fuel(required_fuel(*mass)));
    println!("part 2 - Total fuel = {}", fuel);
}

fn required_fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn total_fuel(fuel: i64) -> i64 {
    let mut total_fuel = fuel;
    let mut current = fuel;
    loop {
        current = required_fuel(current);
        if current <= 0 {
            break
        }
        total_fuel += current
    }
    return total_fuel
}

#[cfg(test)]
mod test {
    use crate::{required_fuel, total_fuel};

    #[test]
    fn test_required_fuel() {
        assert_eq!(required_fuel(12), 2);
        assert_eq!(required_fuel(14), 2);
        assert_eq!(required_fuel(1969), 654);
        assert_eq!(required_fuel(100756), 33583);
    }

    #[test]
    fn test_total_fuel_mass() {
        assert_eq!(total_fuel(2), 2);
        assert_eq!(total_fuel(654), 966);
        assert_eq!(total_fuel(33583), 50346);
    }
}
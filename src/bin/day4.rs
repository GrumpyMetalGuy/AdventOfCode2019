fn get_matches(start: i32, end: i32, restrict_length: bool) -> i32 {
    let mut matches = 0;

    for candidate in start..=end {
        let str_candidate = candidate.to_string();
        let mut rle_data: Vec<i32> = Vec::new();
        let mut str_iterator = str_candidate.chars();
        let mut previous_char = str_iterator.next().unwrap();
        let mut previous_char_count = 1;
        let mut increasing = true;

        for current_char in str_iterator {
            if current_char != previous_char {
                if current_char < previous_char {
                    increasing = false;
                }

                rle_data.push(previous_char_count);
                previous_char = current_char;
                previous_char_count = 1;
            } else {
                previous_char_count += 1;
            }
        }

        rle_data.push(previous_char_count);

        if increasing {
            if restrict_length {
                if rle_data.iter().filter(|&&n| n == 2).count() > 0 {
                    matches += 1;
                }
            } else {
                let counter = rle_data.iter().filter(|&&n| n > 1).count();
                if counter > 0 {
                    println!("{}", str_candidate);
                    matches += 1;
                }
            }
        }
    }

    matches
}

fn main() {
    println!("{}", get_matches(382345, 843167, false));
    println!("{}", get_matches(382345, 843167, true));
}

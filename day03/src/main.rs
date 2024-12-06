mod part2;
use part2::part2;

pub fn reset(
    found_mul: &mut bool,
    mul_index: &mut usize,
    bracket_rc: &mut i32,
    is_in_bracket: &mut bool,
    digit_counter: &mut i32,
    first_number: &mut String,
    second_number: &mut String,
    passed_comma: &mut bool,
    is_complete_inst: &mut bool,
) {
    *found_mul = false;
    *mul_index = 0;
    *bracket_rc = 0;
    *is_in_bracket = false;
    *digit_counter = 0;
    first_number.clear();
    second_number.clear();
    *passed_comma = false;
    *is_complete_inst = false;
}

pub fn find_inst(inst: &str, ch: char, index: &mut usize) -> bool {
    if *index == inst.len()-1 {
        return true;
    } else if *index < inst.len() && ch == inst.chars().nth(*index).unwrap() {
        *index += 1;
    } else {
        *index = 0;
    }
    return false;
}

pub fn in_bracket(ch: char, digit_counter: &mut i32, first_number: &mut String, second_number: &mut String, passed_comma: &mut bool) -> bool {
    if ch.is_digit(10) {
        if *digit_counter < 3 {
            *digit_counter += 1;

            if *passed_comma {
                second_number.push(ch);
            } else {
                first_number.push(ch);
            }
        } else if *digit_counter == 3 {
            return false;
        }
    } else if ch == ',' && !first_number.is_empty() {
        *passed_comma = true;
        *digit_counter = 0;
    } else {
        return false;
    }

    return true;
}

pub fn mul(first: &String, second: &String) -> u64 {
    first.parse::<u64>().unwrap() * second.parse::<u64>().unwrap()
}

fn main() {
    // part 1
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let mut found_mul = false;
    let mut mul_index: usize = 0;

    let mut bracket_rc = 0;
    let mut is_in_bracket = false;

    let mut digit_counter = 0;
    let mut first_number = String::new();
    let mut second_number = String::new();

    let mut passed_comma = false;
    let mut is_complete_inst = false;

    let mut total = 0;
    for input_ch in input.chars() {
        if is_complete_inst {
            if (second_number.is_empty() || second_number.len() > 3) || (first_number.is_empty() || first_number.len() > 3) {
                reset(
                    &mut found_mul,
                    &mut mul_index,
                    &mut bracket_rc,
                    &mut is_in_bracket,
                    &mut digit_counter,
                    &mut first_number,
                    &mut second_number,
                    &mut passed_comma,
                    &mut is_complete_inst
                );
                continue;
            }

            total += mul(&first_number, &second_number);
            reset(
                &mut found_mul,
                &mut mul_index,
                &mut bracket_rc,
                &mut is_in_bracket,
                &mut digit_counter,
                &mut first_number,
                &mut second_number,
                &mut passed_comma,
                &mut is_complete_inst
            );
        }

        if !found_mul {
            found_mul = find_inst("mul", input_ch, &mut mul_index);
            continue;
        }

        if bracket_rc == 0 {
            if input_ch == '(' {
                bracket_rc += 1;
                is_in_bracket = true;
                continue;
            }
        }

        if bracket_rc == 1 {
            if input_ch == ')' {
                bracket_rc -= 1;
                is_in_bracket = false;

                if passed_comma {
                    is_complete_inst = true;
                }
                continue;
            }
        }

        if input_ch == '(' || input_ch == ')' && bracket_rc != 0 || bracket_rc != 1 {
            reset(
                &mut found_mul,
                &mut mul_index,
                &mut bracket_rc,
                &mut is_in_bracket,
                &mut digit_counter,
                &mut first_number,
                &mut second_number,
                &mut passed_comma,
                &mut is_complete_inst
            );
            continue;
        }

        if is_in_bracket {
            if !in_bracket(input_ch, &mut digit_counter, &mut first_number, &mut second_number, &mut passed_comma) {
                reset(
                    &mut found_mul,
                    &mut mul_index,
                    &mut bracket_rc,
                    &mut is_in_bracket,
                    &mut digit_counter,
                    &mut first_number,
                    &mut second_number,
                    &mut passed_comma,
                    &mut is_complete_inst
                );
                continue;
            }
        }
    }
    println!("{total}");

    // part 2
    part2();
}

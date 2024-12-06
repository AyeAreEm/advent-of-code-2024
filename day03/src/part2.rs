use regex::Regex;

// this is not my solution, i couldn't solve part 2 with how i solved part 1... I don't wanna learn
// regex lmao. thanks to @bornsurvivor88 on reddit
pub fn part2() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    fn sum_of_muls(input: &str) -> i32 {
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
            .unwrap()
            .captures_iter(input)
            .map(|cap| cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap())
            .sum()
    }

    let mut segments = input.split("don't()");
    let initial = sum_of_muls(segments.next().unwrap_or(""));
    let part_two = segments.fold(initial, |acc, segment| {
        acc + sum_of_muls(&segment[segment.find("do()").unwrap_or(segment.len())..])
    });

    println!("{:#?}", part_two);
}

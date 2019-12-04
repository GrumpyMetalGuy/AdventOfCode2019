use counter::Counter;

fn increasing(code: i32) -> bool {
    let str_code = code.to_string();
    let code_iterator = str_code.chars();
    let mut next_code_iterator = str_code.chars();
    next_code_iterator.next();

    for pair in code_iterator.zip(next_code_iterator) {
        if pair.0 > pair.1 {
            return false;
        }
    }

    true
}

fn digit_count(code: i32, strict: bool) -> bool {
    let str_code = code.to_string();
    let counter = str_code.chars().collect::<Counter<_>>();

    if strict {
        return counter.values().filter(|&&n| n == 2).count() > 0;
    } else {
        return counter.values().filter(|&&n| n > 1).count() > 0;
    }
}

fn main() {
    let candidates = (382345..=843167).collect::<Vec<_>>();

    let part1 = candidates
        .iter()
        .map(|&n| increasing(n) && digit_count(n, false))
        .filter(|&n| n)
        .count();

    let part2 = candidates
        .iter()
        .map(|&n| increasing(n) && digit_count(n, true))
        .filter(|&n| n)
        .count();

    println!("{}", part1);
    println!("{}", part2);
}

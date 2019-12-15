use num::Integer;
use slice_deque::SliceDeque;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::f64;

type AsteroidLocation = (i64, i64);

fn is_asteroid_obstructed(
    current_asteroid: &AsteroidLocation,
    target_asteroid: &AsteroidLocation,
    asteroids: &HashSet<AsteroidLocation>,
) -> bool {
    let mut x_delta = current_asteroid.0 - target_asteroid.0;
    let mut y_delta = current_asteroid.1 - target_asteroid.1;

    if x_delta == 0 {
        y_delta /= y_delta.abs();
    } else if y_delta == 0 {
        x_delta /= x_delta.abs()
    } else {
        let gcd = x_delta.gcd(&y_delta);

        x_delta /= gcd;
        y_delta /= gcd;
    }

    let mut current_x = current_asteroid.0;
    let mut current_y = current_asteroid.1;
    let mut intervening_asteroid_found = false;

    while !(current_x == target_asteroid.0 && current_y == target_asteroid.1) {
        current_x -= x_delta;
        current_y -= y_delta;

        if asteroids.contains(&(current_x, current_y)) {
            let intervening_asteroid = asteroids.get(&(current_x, current_y)).unwrap();

            intervening_asteroid_found = intervening_asteroid != target_asteroid;
            break;
        }
    }

    intervening_asteroid_found
}

fn calculate_distance(first: &AsteroidLocation, second: &AsteroidLocation) -> f64 {
    (((second.0 - first.0).pow(2) + (second.1 - first.1).pow(2)) as f64).sqrt()
}

fn main() {
    let input: Vec<&str> = vec![
        "#..#.#.###.#...##.##....",
        ".#.#####.#.#.##.....##.#",
        "##..#.###..###..#####..#",
        "####.#.#..#....#..##.##.",
        ".#######.#####...#.###..",
        ".##...#.#.###..###.#.#.#",
        ".######.....#.###..#....",
        ".##..##.#..#####...###.#",
        "#######.#..#####..#.#.#.",
        ".###.###...##.##....##.#",
        "##.###.##.#.#..####.....",
        "#.#..##..#..#.#..#####.#",
        "#####.##.#.#.#.#.#.#..##",
        "#...##.##.###.##.#.###..",
        "####.##.#.#.####.#####.#",
        ".#..##...##..##..#.#.##.",
        "###...####.###.#.###.#.#",
        "..####.#####..#####.#.##",
        "..###..###..#..##...#.#.",
        "##.####...##....####.##.",
        "####..#..##.#.#....#..#.",
        ".#..........#..#.#.####.",
        "###..###.###.#.#.#....##",
        "########.#######.#.##.##",
    ];

    let input_test: Vec<&str> = vec![
        ".#..##.###...#######",
        "##.############..##.",
        ".#.######.########.#",
        ".###.#######.####.#.",
        "#####.##.#.##.###.##",
        "..#####..#.#########",
        "####################",
        "#.####....###.#.#.##",
        "##.#################",
        "#####.##.###..####..",
        "..######..##.#######",
        "####.##.####...##..#",
        ".#####..#.######.###",
        "##...#.##########...",
        "#.##########.#######",
        ".####.#.###.###.#.##",
        "....##.##.###..#####",
        ".#.#.###########.###",
        "#.#.#.#####.####.###",
        "###.##.####.##.#..##",
    ];

    let mut asteroids: HashSet<AsteroidLocation> = HashSet::new();
    let mut line_count = 0;

    for line in input {
        for (x, value) in line.chars().enumerate() {
            if value == '#' {
                asteroids.insert((x as i64, line_count));
            }
        }

        line_count += 1;
    }

    let mut best_asteroid_count = 0;
    let mut best_asteroid_position = asteroids.iter().next().unwrap();

    for prime_asteroid in &asteroids {
        let mut current_asteroid_count = 0;

        for other_asteroid in &asteroids {
            if other_asteroid == prime_asteroid {
                continue;
            }

            if !is_asteroid_obstructed(other_asteroid, prime_asteroid, &asteroids) {
                current_asteroid_count += 1;
            }
        }

        if current_asteroid_count > best_asteroid_count {
            best_asteroid_count = current_asteroid_count;
            best_asteroid_position = prime_asteroid;
        }
    }

    println!("{}, {:?}", best_asteroid_count, best_asteroid_position);

    // Part 2
    let prime_asteroid = *(asteroids.get(&best_asteroid_position).unwrap());

    let mut target_count = 0;

    let mut nearest_asteroid_structure: BTreeMap<i64, SliceDeque<AsteroidLocation>> =
        BTreeMap::new();

    for asteroid in &asteroids {
        if asteroid.0 == prime_asteroid.0 && asteroid.1 == prime_asteroid.1 {
            continue;
        }

        let mut angle =
            ((asteroid.1 - prime_asteroid.1) as f64).atan2((asteroid.0 - prime_asteroid.0) as f64);

        angle += f64::consts::PI / 2.0;

        angle = angle.to_degrees();

        if angle < 0.0 {
            angle += 360.0;
        }

        let rounded_angle = (angle * 10000000.0) as i64;

        if !nearest_asteroid_structure.contains_key(&rounded_angle) {
            nearest_asteroid_structure.insert(rounded_angle, SliceDeque::<AsteroidLocation>::new());
        };

        nearest_asteroid_structure
            .get_mut(&rounded_angle)
            .unwrap()
            .push_back(*asteroid);

        nearest_asteroid_structure
            .get_mut(&rounded_angle)
            .unwrap()
            .sort_by(|first, second| {
                if calculate_distance(first, &prime_asteroid)
                    < calculate_distance(second, &prime_asteroid)
                {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            })
    }

    loop {
        for (_angle, distances) in &mut nearest_asteroid_structure {
            if distances.is_empty() {
                continue;
            }

            let asteroid = &distances.pop_front().unwrap();

            target_count += 1;

            if target_count == 200 {
                println!("BANG - {}", asteroid.0 * 100 + asteroid.1);
            } else {
                println!("{}, {}", asteroid.0, asteroid.1);
            }
        }
    }
}

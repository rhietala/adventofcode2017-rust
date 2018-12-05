#![allow(dead_code)]

extern crate regex;

use std::collections::HashSet;
use self::regex::Regex;

/**
*/

type Coord = (u32, u32);

fn part1and2(input: Vec<String>) -> (u32, String) {
    let mut claimed: HashSet<Coord> = HashSet::new();
    let mut duplicates: HashSet<Coord> = HashSet::new();
    let mut id: String = String::from("");

    // this could've been done with HashSet union and intersection,
    // but couldn't get the types/references to work :(

    for s in input.clone() {
        let current = parse_claim(&s);
        for coord in current.1 {
            if claimed.contains(&coord) {
                duplicates.insert(coord);
            } else {
                claimed.insert(coord);
            }
        }
    }

    // for part 2, loop all again to find the one that doesn't have
    // duplicates
    //
    // this is a bit of an ugly solution

    for s in input {
        let current = parse_claim(&s);
        let mut duplicate_found: bool = false;
        for coord in current.1 {
            if duplicates.contains(&coord) {
                duplicate_found = true;
            }
        }
        if !duplicate_found {
            id = current.0.clone();
            println!("no duplicate found on id {}", id);
        }
    }


    (duplicates.len() as u32, id)
}

fn parse_claim(s: &String) -> (String, HashSet<Coord>) {
    let mut coordinates: HashSet<Coord> = HashSet::new();
    let re = Regex::new(r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)").unwrap();
    let caps = re.captures(s).unwrap();
    let left = caps.name("left").unwrap().as_str().parse::<u32>().unwrap();
    let top = caps.name("top").unwrap().as_str().parse::<u32>().unwrap();
    let width = caps.name("width").unwrap().as_str().parse::<u32>().unwrap();
    let height = caps.name("height").unwrap().as_str().parse::<u32>().unwrap();

    for x in left..(left + width) {
        for y in top..(top + height) {
            coordinates.insert((x, y));
        }
    }

    (String::from(caps.name("id").unwrap().as_str()), coordinates)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use ::helper;

    fn part1_test_input() -> Vec<String> {
        vec!(
            String::from("#1 @ 1,3: 4x4"),
            String::from("#2 @ 3,1: 4x4"),
            String::from("#3 @ 5,5: 2x2")
        )
    }

    #[test]
    fn parse_claim() {
        let mut expected: HashSet<super::Coord> = HashSet::new();
        for x in 1..5 {
            for y in 3..7 {
                expected.insert((x, y));
            }
        }
        assert_eq!(super::parse_claim(part1_test_input().get(0).unwrap()),
                   (String::from("1"), expected));
    }

    #[test]
    fn part1test1() {
        assert_eq!(super::part1and2(part1_test_input()).0, 4);
    }

    #[test] // this is a bit slow one
    fn part1() {
        assert_eq!(super::part1and2(helper::read_file_string("input/day03.txt")).0, 109716);
    }

    #[test]
    fn part2test1() {
        assert_eq!(super::part1and2(part1_test_input()).1, "3");
    }

    #[test] // this is a bit slow one
    fn part2() {
        assert_eq!(super::part1and2(helper::read_file_string("input/day03.txt")).1, String::from("124"));
    }
}

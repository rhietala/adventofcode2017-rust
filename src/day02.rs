#![allow(dead_code)]

use std::collections::HashMap;

/**
    To make sure you didn't miss any, you scan the likely candidate boxes
    again, counting the number that have an ID containing exactly two of
    any letter and then separately counting those with exactly three of
    any letter. You can multiply those two counts together to get a
    rudimentary checksum and compare it to what your device predicts.
 */

fn part1(input: Vec<String>) -> u32 {
    let doubles_and_triples: Vec<(u32, u32)> = input.iter().map(|s| has_doubles_or_triples(s)).collect();
    let doubles: u32 = doubles_and_triples.iter().map(|t| t.0).sum();
    let triples: u32 = doubles_and_triples.iter().map(|t| t.1).sum();

    doubles * triples
}

fn has_doubles_or_triples(input: &String) -> (u32, u32) {
    let occurrences = count_occurrences(input);
    let mut doubles: u32 = 0;
    let mut triples: u32 = 0;
    for c in occurrences.values() {
        if *c == 2 {
            doubles = 1;
        } else if *c == 3 {
            triples = 1;
        }
    }
    (doubles, triples)
}

fn count_occurrences(input: &String) -> HashMap<char, u32> {
    let mut occurrences: HashMap<char, u32> = HashMap::new();
    for c in input.chars() {
        let count = occurrences.entry(c).or_insert(0);
        *count += 1;
    }
    occurrences
}

/**
    The boxes will have IDs which differ by exactly one character at the same
    position in both strings.
*/

fn part2(input: Vec<String>) -> String {
    if input.len() == 0 || input.get(0).unwrap().len() == 0 {
        return String::from("");
    }

    for n in 0..input.get(0).unwrap().len() {
        let removed: Vec<String> =
            input.clone().iter().map(|s| remove_nth(s, n)).collect();
        match find_duplicate(removed) {
            Some(x) => return x,
            None => ()
        }
    }
    String::from("")
}

fn find_duplicate(xs: Vec<String>) -> Option<String> {
    use std::collections::HashSet;
    let mut ys: HashSet<String> = HashSet::new();
    for x in xs {
        if ys.contains(&x) {
            return Some(x);
        } else {
            ys.insert(x);
        }
    }
    None
}

fn remove_nth(input: &String, n: usize) -> String {
    let mut s = input.clone();
    s.remove(n);
    s
}


#[cfg(test)]
mod tests {
    use ::helper;

    fn part1_test_input() -> Vec<String> {
        vec!(
            String::from("abcdef"),
            String::from("bababc"),
            String::from("abbcde"),
            String::from("abcccd"),
            String::from("aabcdd"),
            String::from("abcdee"),
            String::from("ababab")
        )
    }

    fn part2_test_input() -> Vec<String> {
        vec!(
            String::from("abcde"),
            String::from("fghij"),
            String::from("klmno"),
            String::from("pqrst"),
            String::from("fguij"),
            String::from("axcye"),
            String::from("wvxyz")
        )
    }

    #[test]
    fn has_doubles1() {
        assert_eq!(super::has_doubles_or_triples(part1_test_input().get(0).unwrap()), (0, 0));
    }

    #[test]
    fn has_doubles2() {
        assert_eq!(super::has_doubles_or_triples(part1_test_input().get(1).unwrap()), (1, 1));
    }

    #[test]
    fn has_doubles3() {
        assert_eq!(super::has_doubles_or_triples(part1_test_input().get(2).unwrap()), (1, 0));
    }

    #[test]
    fn has_doubles4() {
        assert_eq!(super::has_doubles_or_triples(part1_test_input().get(3).unwrap()), (0, 1));
    }

    #[test]
    fn has_doubles5() {
        assert_eq!(super::has_doubles_or_triples(part1_test_input().get(4).unwrap()), (1, 0));
    }

    #[test]
    fn has_doubles6() {
        assert_eq!(super::has_doubles_or_triples(part1_test_input().get(5).unwrap()), (1, 0));
    }

    #[test]
    fn has_doubles7() {
        assert_eq!(super::has_doubles_or_triples(part1_test_input().get(6).unwrap()), (0, 1));
    }

    #[test]
    fn checksum() {
        assert_eq!(super::part1(part1_test_input()), 12);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(helper::read_file_string("input/day02.txt")), 8118);
    }

    #[test]
    fn remove_nth() {
        assert_eq!(super::remove_nth(&String::from("foo"), 2), String::from("fo"));
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::part2(part2_test_input()), String::from("fgij"));
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(helper::read_file_string("input/day02.txt")), String::from("jbbenqtlaxhivmwyscjukztdp"));
    }
}

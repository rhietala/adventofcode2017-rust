#![allow(dead_code)]

/**
   A value like +6 means the current frequency increases by 6; a value
   like -3 means the current frequency decreases by 3.

   Starting with a frequency of zero, what is the resulting frequency
   after all of the changes in frequency have been applied?
*/

fn part1(input: Vec<i32>) -> i32 {
    input.iter().sum()
}

/**
   You notice that the device repeats the same frequency change list over
   and over. To calibrate the device, you need to find the first frequency
   it reaches twice.

   Note that your device might need to repeat its list of frequency changes
   many times before a duplicate frequency is found, and that duplicates
   might be found while in the middle of processing the list.
*/

fn part2(input: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut cur: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();

    loop {
        for i in &input {
            if visited.contains(&cur) {
                return cur;
            } else {
                visited.insert(cur);
            }
            cur += i;
        }
    }
}

#[cfg(test)]
mod tests {
    use ::helper;

    #[test]
    fn part1test1() {
        assert_eq!(super::part1(vec!(1, -2, 3, 1)), 3);
    }

    #[test]
    fn part1test2() {
        assert_eq!(super::part1(vec!(1, 1, 1)), 3);
    }

    #[test]
    fn part1test3() {
        assert_eq!(super::part1(vec!(1, 1, -2)), 0);
    }

    #[test]
    fn part1test4() {
        assert_eq!(super::part1(vec!(-1, -2, -3)), -6);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(helper::read_file_i32("input/day01.txt")), 497);
    }

    #[test]
    fn part2test1() {
        assert_eq!(super::part2(vec!(1, -1)), 0);
    }

    #[test]
    fn part2test2() {
        assert_eq!(super::part2(vec!(3, 3, 4, -2, -4)), 10);
    }

    #[test]
    fn part2test3() {
        assert_eq!(super::part2(vec!(-6, 3, 8, 5, -6)), 5);
    }

    #[test]
    fn part2test4() {
        assert_eq!(super::part2(vec!(7, 7, -2, -7, -4)), 14);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(helper::read_file_i32("input/day01.txt")), 558);
    }
}

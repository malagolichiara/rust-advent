#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

lazy_static! {
    static ref RULE_REGEX: Regex =
        Regex::new(r"^(.+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Rule {
    name: String,
    ranges: (RangeInclusive<u64>, RangeInclusive<u64>),
}

impl Rule {
    fn contains(&self, n: &u64) -> bool {
        self.ranges.0.contains(n) || self.ranges.1.contains(n)
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let capture = RULE_REGEX.captures(line).unwrap();

        let name = String::from(&capture[1]);
        let range_1_start: u64 = capture[2].parse().unwrap();
        let range_1_end: u64 = capture[3].parse().unwrap();
        let range_2_start: u64 = capture[4].parse().unwrap();
        let range_2_end: u64 = capture[5].parse().unwrap();

        Ok(Rule {
            name,
            ranges: (
                RangeInclusive::new(range_1_start, range_1_end),
                RangeInclusive::new(range_2_start, range_2_end),
            ),
        })
    }
}

pub fn part1(input: &str) -> u32 {
    let mut i = input.split("\n\n");
    let unparsed_rules = i.next().unwrap();
    let _own_ticket = i.next().unwrap();
    let other_tickets = i.next().unwrap();

    let rules: Vec<Rule> = unparsed_rules.lines().map(|l| l.parse().unwrap()).collect();
    let mut failed_numbers: u32 = 0;
    for ticket in other_tickets.lines().skip(1) {
        for number in ticket.split(',') {
            let n: u64 = number.parse().unwrap();
            let found = rules.iter().find(|rule| rule.contains(&n));
            if found.is_none() {
                failed_numbers += n as u32;
            }
        }
    }

    failed_numbers
}

pub fn part2(input: &str) -> u64 {
    let mut i = input.split("\n\n");
    let unparsed_rules = i.next().unwrap();
    let own_ticket = i.next().unwrap();
    let other_tickets = i.next().unwrap();

    let rules: Vec<Rule> = unparsed_rules.lines().map(|l| l.parse().unwrap()).collect();

    let own_ticket_values: Vec<u64> = own_ticket
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();

    let other_tickets_values: Vec<Vec<u64>> = other_tickets
        .lines()
        .skip(1)
        .filter_map(|line| {
            let ticket: Vec<u64> = line.split(',').map(|l| l.parse().unwrap()).collect();
            for number in &ticket {
                let found = rules.iter().find(|rule| rule.contains(number));
                if found.is_none() {
                    return None;
                }
            }
            Some(ticket)
        })
        .collect();

    let num_fields = own_ticket_values.len();

    let mut guesses: Vec<HashSet<&Rule>> =
        (0..num_fields).map(|_| rules.iter().collect()).collect();

    for ticket in other_tickets_values {
        for (col, val) in ticket.iter().enumerate() {
            let set = guesses.get_mut(col).unwrap();
            let cloned_set = set.clone();
            let rules_to_remove: Vec<&&Rule> = cloned_set
                .iter()
                .filter(|rule| !rule.contains(val))
                .collect();

            for rule in rules_to_remove {
                set.remove(rule);
            }
        }
    }

    // At this point we expect that there will be at least 1 set with only one column
    // We can remove that column from all the other sets.
    // At that point we expect that there will be other sets with only 1 column, so we keep
    // removing that column from all the other sets.
    // We keep going until all the sets are of length 1 (or at least the ones with `departure*`
    // in the name)
    let mut sure_columns: HashSet<&Rule> = HashSet::new();
    let start_column = guesses
        .iter()
        .find(|rules| rules.len() == 1)
        .expect("There needs to be at least 1!");
    sure_columns.insert(start_column.iter().next().unwrap());
    loop {
        for i in 0..guesses.len() {
            let columns = guesses.get_mut(i).unwrap();
            if columns.len() > 1 {
                for col in sure_columns.iter() {
                    columns.remove(col);
                }
            }
            if columns.len() == 1 {
                sure_columns.insert(columns.iter().next().unwrap());
            }
        }
        if sure_columns.len() == guesses.len() {
            break;
        }
    }

    let guess_names: Vec<&String> = guesses
        .iter()
        .map(|s| s.iter().map(|r| &(r.name)).next().unwrap())
        .collect();

    let mut result = 1;
    for (i, name) in guess_names.iter().enumerate() {
        if name.starts_with("departure") {
            result *= own_ticket_values.get(i).unwrap();
        }
    }

    result
}

#[cfg(test)]
mod ex16_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 21996);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 650080463519);
    }
}

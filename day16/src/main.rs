use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
    num::ParseIntError,
    ops::RangeInclusive,
    str::FromStr,
};

use anyhow::anyhow;

type TicketField = u64;

#[derive(Clone, Debug)]
struct Ticket {
    fields: Vec<TicketField>,
}

impl FromStr for Ticket {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<Vec<TicketField>, _>>()?;
        let ticket = Ticket { fields };
        Ok(ticket)
    }
}

impl Ticket {
    fn is_valid(&self, rules: &[Rule]) -> bool {
        self.fields
            .iter()
            .any(|field| rules.iter().all(|rule| !rule.is_match(field)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule<'a> {
    name: &'a str,
    ranges: [RangeInclusive<TicketField>; 2],
}

impl<'a> TryFrom<&'a str> for Rule<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parts = value.split(':').map(str::trim);
        let name = parts.next().ok_or_else(|| anyhow!("parsing field name"))?;
        let ranges = parts
            .next()
            .and_then(|v| {
                v.split(" or ")
                    .map(|r| {
                        r.split('-')
                            .map(str::parse)
                            .collect::<Result<Vec<TicketField>, _>>()
                            .map(|r| (r[0]..=r[1]))
                            .ok()
                    })
                    .collect::<Option<Vec<RangeInclusive<TicketField>>>>()
            })
            .ok_or_else(|| anyhow!("parsing rules"))?;
        let rule = Rule {
            name,
            ranges: ranges.try_into().unwrap(),
        };
        Ok(rule)
    }
}

impl Rule<'_> {
    fn is_match(&self, field: &TicketField) -> bool {
        self.ranges.iter().any(|rule| rule.contains(field))
    }
}

fn part1(rules: &[Rule], nearby_tickets: &[Ticket]) -> Vec<Ticket> {
    let (invalid_tickets, valid_tickets): (Vec<Ticket>, Vec<Ticket>) = nearby_tickets
        .iter()
        .cloned()
        .partition(|ticket| ticket.is_valid(rules));

    let error_rate = invalid_tickets
        .iter()
        .flat_map(|ticket| {
            ticket
                .fields
                .iter()
                .filter(|field| rules.iter().all(|rule| !rule.is_match(field)))
        })
        .sum::<TicketField>();
    println!("Part 1: {}", error_rate);

    valid_tickets
}

fn part2(rules: &[Rule], valid_tickets: &[Ticket], my_ticket: &Ticket) {
    let fields_length = my_ticket.fields.len();
    let mut field_matches = (0..fields_length)
        .map(|field_index| {
            let matching_rules = rules
                .iter()
                .enumerate()
                .filter(|(_, rule)| {
                    valid_tickets
                        .iter()
                        .all(|ticket| rule.is_match(&ticket.fields[field_index]))
                })
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            (field_index, matching_rules)
        })
        .collect::<Vec<_>>();
    field_matches.sort_by_key(|(_, matching_rules)| matching_rules.len());

    let (rule_indicies, _) = field_matches.iter().fold(
        (vec![0; fields_length], HashSet::new()),
        |(mut rule_incicies, mut used_rules), (field_index, rules)| {
            let candidate_rules: HashSet<_> = rules.iter().copied().collect();
            let next = *candidate_rules.difference(&used_rules).last().unwrap();
            used_rules.insert(next);
            rule_incicies[*field_index] = next;
            (rule_incicies, used_rules)
        },
    );

    let answer = rules
        .iter()
        .enumerate()
        .filter(|(_, r)| r.name.starts_with("departure"))
        .map(|(rule_index, _)| {
            let field_index = rule_indicies.iter().position(|&i| i == rule_index).unwrap();
            my_ticket.fields[field_index]
        })
        .product::<TicketField>();

    println!("Part 2: {}", answer);
}

fn main() -> anyhow::Result<()> {
    let mut parts = include_str!("../input.txt").split("\n\n");
    let rules = parts
        .next()
        .and_then(|part| {
            part.lines()
                .map(Rule::try_from)
                .collect::<Result<Vec<Rule>, _>>()
                .ok()
        })
        .ok_or_else(|| anyhow!("parsing rules"))?;
    let my_ticket: Ticket = parts
        .next()
        .and_then(|part| part.lines().nth(1).and_then(|line| line.parse().ok()))
        .ok_or_else(|| anyhow!("parsing my ticket"))?;
    let nearby_tickets: Vec<Ticket> = parts
        .next()
        .and_then(|part| part.lines().skip(1).map(|line| line.parse().ok()).collect())
        .ok_or_else(|| anyhow!("parsing nearby tickets"))?;

    let valid_tickets = part1(&rules, &nearby_tickets);
    part2(&rules, &valid_tickets, &my_ticket);

    Ok(())
}

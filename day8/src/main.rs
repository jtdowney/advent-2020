use std::{
    collections::HashSet,
    io::{self, BufRead},
    str::FromStr,
};

use anyhow::{anyhow, bail, Context, Result};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    Increment,
    NoOperation,
    Jump,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = match s {
            "acc" => Instruction::Increment,
            "nop" => Instruction::NoOperation,
            "jmp" => Instruction::Jump,
            _ => bail!("invalid instruction"),
        };

        Ok(instruction)
    }
}

#[derive(Debug, Copy, Clone)]
struct Operation {
    instruction: Instruction,
    argument: i16,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let instruction = parts
            .next()
            .ok_or_else(|| anyhow!("No instruction"))
            .and_then(|i| i.parse())?;
        let argument = parts
            .next()
            .ok_or_else(|| anyhow!("No argument"))
            .and_then(|i| i.parse().context("Failed to parse argument"))?;

        let operation = Operation {
            instruction,
            argument,
        };

        Ok(operation)
    }
}

fn part1(operations: &[Operation]) {
    let mut accumulator = 0;
    let mut ip = 0;
    let mut seen = HashSet::new();
    loop {
        let op = operations[ip];
        if seen.contains(&ip) {
            break;
        } else {
            seen.insert(ip);
        }

        match op.instruction {
            Instruction::Increment => {
                accumulator += op.argument;
            }
            Instruction::NoOperation => {}
            Instruction::Jump => {
                ip = (ip as i16 + op.argument) as usize;
            }
        };

        if op.instruction != Instruction::Jump {
            ip += 1;
        }
    }

    println!("Part 1: {}", accumulator);
}

fn part2(operations: &[Operation]) {
    let candidates = (0..operations.len())
        .filter_map(|n| {
            let op = operations[n];
            let instruction = match op.instruction {
                Instruction::Increment => return None,
                Instruction::NoOperation => Instruction::Jump,
                Instruction::Jump => Instruction::NoOperation,
            };

            let mut candidate = operations.to_vec();
            candidate[n].instruction = instruction;

            Some(candidate)
        })
        .collect::<Vec<Vec<Operation>>>();

    for candidate in candidates {
        let mut accumulator = 0;
        let mut ip = 0;
        let mut count = vec![0u8; candidate.len()];
        loop {
            let op = candidate[ip];
            count[ip] += 1;
            if count[ip] > 25 {
                break;
            }

            match op.instruction {
                Instruction::Increment => {
                    accumulator += op.argument as i64;
                }
                Instruction::NoOperation => {}
                Instruction::Jump => {
                    ip = (ip as i16 + op.argument) as usize;
                }
            };

            if op.instruction != Instruction::Jump {
                ip += 1;
            }

            if ip >= candidate.len() {
                println!("Part 2: {}", accumulator);
                return;
            }
        }
    }
}

fn main() -> Result<()> {
    let operations = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.context("Failed to read line")
                .and_then(|op| op.parse())
        })
        .collect::<Result<Vec<Operation>>>()?;

    part1(&operations);
    part2(&operations);

    Ok(())
}

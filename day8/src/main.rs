use std::{
    collections::HashSet,
    io::{self, BufRead},
    str::FromStr,
};

use anyhow::{anyhow, bail, Context, Result};

#[derive(Copy, Clone, Default)]
struct Environment {
    accumulator: i64,
    ip: usize,
}

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone)]
struct Operation {
    instruction: Instruction,
    argument: i16,
}

impl Operation {
    fn step(&self, mut env: Environment) -> Environment {
        match self.instruction {
            Instruction::Increment => {
                env.accumulator += self.argument as i64;
            }
            Instruction::NoOperation => {}
            Instruction::Jump => {
                env.ip = (env.ip as i16 + self.argument) as usize;
            }
        };

        if self.instruction != Instruction::Jump {
            env.ip += 1;
        }

        env
    }
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
    let mut env = Environment::default();
    let mut seen = HashSet::new();
    loop {
        if seen.contains(&env.ip) {
            break;
        } else {
            seen.insert(env.ip);
        }

        let op = operations[env.ip];
        env = op.step(env);
    }

    println!("Part 1: {}", env.accumulator);
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

    for operations in candidates {
        let mut env = Environment::default();
        let mut count = vec![0u8; operations.len()];
        loop {
            count[env.ip] += 1;
            if count[env.ip] > 25 {
                break;
            }

            let op = operations[env.ip];
            env = op.step(env);

            if env.ip >= operations.len() {
                println!("Part 2: {}", env.accumulator);
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

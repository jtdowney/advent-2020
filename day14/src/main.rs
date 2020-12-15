use std::{collections::HashMap, convert::TryFrom, str};

use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
enum Instruction<'a> {
    Mask(&'a str),
    Memory(u64, u64),
}

#[derive(Debug, Default)]
struct State<'a> {
    mask: &'a str,
    memory: HashMap<u64, u64>,
}

impl<'a> TryFrom<&'a str> for Instruction<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let parts = value.split('=').map(str::trim).collect::<Vec<&str>>();
        let instruction = if parts[0].starts_with("mem[") {
            let end = parts[0]
                .bytes()
                .position(|b| b == b']')
                .ok_or_else(|| anyhow!("unable to find valid memory address"))?;
            let address = parts[0][4..end].parse().context("parsing address")?;
            let value = parts[1].parse().context("parsing value")?;
            Instruction::Memory(address, value)
        } else {
            let value = parts[1];
            Instruction::Mask(value)
        };

        Ok(instruction)
    }
}

fn part1(instructions: &[Instruction]) {
    let state =
        instructions.iter().fold(
            State::default(),
            |mut state, instruction| match instruction {
                Instruction::Mask(m) => {
                    state.mask = m;
                    state
                }
                Instruction::Memory(address, value) => {
                    let value_bits = format!("{:036b}", value);
                    let result = state
                        .mask
                        .chars()
                        .zip(value_bits.chars())
                        .map(|(m, v)| if m == 'X' { v } else { m })
                        .collect::<String>();

                    let masked_value = u64::from_str_radix(&result, 2).unwrap();
                    *state.memory.entry(*address).or_default() = masked_value;
                    state
                }
            },
        );

    let answer = state.memory.values().sum::<u64>();
    println!("Part 1: {}", answer);
}

fn part2(instructions: &[Instruction]) {
    let state =
        instructions.iter().fold(
            State::default(),
            |mut state, instruction| match instruction {
                Instruction::Mask(m) => {
                    state.mask = m;
                    state
                }
                Instruction::Memory(address, value) => {
                    let address_bits = format!("{:036b}", address);
                    let template = state
                        .mask
                        .chars()
                        .zip(address_bits.chars())
                        .map(|(m, v)| if m == '0' { v } else { m })
                        .collect::<String>();
                    let floating_positions = template
                        .chars()
                        .enumerate()
                        .filter(|&(_, ch)| ch == 'X')
                        .map(|(i, _)| i)
                        .collect::<Vec<usize>>();
                    let max = 2u32.pow(floating_positions.len() as u32);
                    (0..max)
                        .filter_map(|n| {
                            let bits = format!("{:032b}", n)
                                .chars()
                                .rev()
                                .take(floating_positions.len())
                                .enumerate()
                                .map(|(i, c)| (floating_positions[i], c))
                                .collect::<HashMap<usize, char>>();
                            let address = template
                                .chars()
                                .enumerate()
                                .map(|(p, ch)| if ch == 'X' { bits[&p] } else { ch })
                                .collect::<String>();
                            u64::from_str_radix(&address, 2).ok()
                        })
                        .for_each(|address| {
                            *state.memory.entry(address).or_default() = *value;
                        });
                    state
                }
            },
        );

    let answer = state.memory.values().sum::<u64>();
    println!("Part 2: {}", answer);
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt")
        .lines()
        .map(Instruction::try_from)
        .collect::<Result<Vec<Instruction>>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

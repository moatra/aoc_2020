use anyhow::{Result, anyhow, bail};
use thiserror::Error;
use bit_vec::BitVec;
use aoc_2020::parse_input;
use std::num::TryFromIntError;
use std::convert::TryFrom;


#[derive(Debug, Clone, Eq, PartialEq)]
enum Instruction {
    Nop(isize),
    Acc(i32),
    Jmp(isize)
}

impl Instruction {
    fn parse(line: &str) -> Result<Instruction> {
        match &line.split_whitespace().collect::<Vec<_>>()[..] {
            &["nop", off] => {
                let offset = off.parse()?;
                Ok(Instruction::Nop(offset))
            },
            &["acc", off] => {
                let offset = off.parse()?;
                Ok(Instruction::Acc(offset))
            },
            &["jmp", off] => {
                let offset = off.parse()?;
                Ok(Instruction::Jmp(offset))
            }
            _ => Err(anyhow!("Unrecognized instruction")),
        }
    }
}

#[derive(Error, Debug)]
enum ExecutionError {
    #[error("execution failed due to infinite loop (next instr: {next_instruction}, current accum: {acc_state})")]
    LoopDetected {
        next_instruction: usize,
        acc_state: i32
    },
    #[error("execution exited with an unexpected instruction pointer (should be: {instr_len}, pointer was: {next_instruction})")]
    InvalidExitPointer {
        next_instruction: usize,
        instr_len: usize
    },
    #[error("error during pointer arithmetic")]
    PointerLogicError {
        #[from]
        from: TryFromIntError
    }
}

fn execute(instructions: &Vec<Instruction>) -> std::result::Result<i32, ExecutionError> {
    let mut loop_detect = BitVec::from_elem(instructions.len(), false);
    let mut accum = 0;
    let mut instr_ptr= 0;
    while let Some(instr) = instructions.get(instr_ptr) {
        if loop_detect.get(instr_ptr).expect("vec lengths should be the same") {
            return Err(ExecutionError::LoopDetected { next_instruction: instr_ptr, acc_state: accum });
        }
        loop_detect.set(instr_ptr, true);
        match instr {
            Instruction::Nop(_) => {
                instr_ptr += 1;
            },
            Instruction::Acc(value) => {
                instr_ptr += 1;
                accum += value;
            },
            Instruction::Jmp(offset) => {
                let ptr = isize::try_from(instr_ptr)?;
                instr_ptr = usize::try_from(ptr + offset)?;
            }
        }
    }
    if instr_ptr == instructions.len() {
        Ok(accum)
    } else {
        Err(ExecutionError::InvalidExitPointer {
            next_instruction: instr_ptr,
            instr_len: instructions.len()
        })
    }
}

fn try_mutate(instructions: &mut Vec<Instruction>) -> Result<i32> {
    for idx in 0..instructions.len() {
        let original_instruction = &instructions[idx];
        let updated_instruction = match original_instruction {
            Instruction::Nop(offset) => Instruction::Jmp(*offset),
            Instruction::Jmp(offset) => Instruction::Nop(*offset),
            _ => continue,
        };
        let original_instruction = std::mem::replace(&mut instructions[idx], updated_instruction);
        if let Ok(acc) = execute(instructions) {
            return Ok(acc);
        }
        instructions[idx] = original_instruction;
    }
    bail!("Couldn't mutate instruction set to success");
}

fn main() -> Result<()> {
    let mut instructions = parse_input(8, |line| Instruction::parse(line))?;

    let result = execute(&instructions);
    dbg!(result);

    dbg!(try_mutate(&mut instructions));
    Ok(())
}
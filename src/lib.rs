use std::path::PathBuf;
use std::fs::File;
use thiserror::Error;
use std::result::Result;

const SRC_ROOT: &str = "/home/thomas/Projects/aoc_2020/src";
// const SRC_ROOT: &str = "/Users/thomas/Work/aoc_2020/src";

pub fn load_input(day: i32) -> std::io::Result<File> {
    let mut path = PathBuf::from(SRC_ROOT);
    let file = format!("day{}/input.txt", day);
    path.push(&file);
    File::open(path)
}

pub fn read_input(day: i32) -> std::io::Result<String> {
    let mut path = PathBuf::from(SRC_ROOT);
    let file = format!("day{}/input.txt", day);
    path.push(&file);
    std::fs::read_to_string(&path)
}

#[derive(Error, Debug)]
pub enum LibError {
    #[error("puzzle input not found: {0}")]
    MissingFile(String),
    #[error("error parsing line {line_num} in file {file}: \"{line}\"")]
    InputError {
        line_num: usize,
        line: String,
        file: String,
        source: anyhow::Error
    }
}

// todo: allow taking custom split arguments
// todo: refactor to use try_into
pub fn parse_input<U, E : Into<anyhow::Error>, F : Fn(&str) -> std::result::Result<U, E>>(day: i32, f: F) -> Result<Vec<U>, LibError> {
    let mut path = PathBuf::from(SRC_ROOT);
    let file = format!("day{}/input.txt", day);
    path.push(&file);
    // let contents = match std::fs::read_to_string(&path) {
    //     Ok(contents) => Ok(contents),
    //     Err(_) => Err(LibError::MissingFile(file.clone())),
    // }?;

    let contents = std::fs::read_to_string(&path).map_err(|_| LibError::MissingFile(file.clone()))?; // todo: avoid clone

    let results: Result<Vec<U>, LibError> = contents.lines().enumerate().map(|(line_num, line)| {
        f(line).map_err(|err| LibError::InputError {
            line_num,
            line: line.to_string(),
            file: file.clone(),
            source: err.into()
        })
    }).collect();
    results
}
//
// pub fn input_into<T : TryFrom<&'_ str>>(day: i32) -> Result<Vec<T>, LibError> {
//     input_into_sep(day, "\n")
// }
//
// pub fn input_into_sep<T : TryFrom<&'_ str>>(day: i32, sep: &str) -> Result<Vec<T>, LibError> {
//     let mut path = PathBuf::from(SRC_ROOT);
//     let file = format!("day{}/input.txt", day);
//     path.push(&file);
//
//     let contents = std::fs::read_to_string(&path).map_err(|_| LibError::MissingFile(file.clone()))?; // todo: avoid clone
//
//     let results: Result<Vec<T>, LibError> = contents.split(sep).enumerate().map(|(line_num, line)| {
//         T::try_from(line).map_err(|err| LibError::InputError {
//             line_num,
//             line: line.to_string(),
//             file: file.clone(),
//             source: err.into()
//         })
//     }).collect();
//     results
// }
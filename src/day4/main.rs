use aoc_2020::read_input;
use anyhow::{bail, Result};
use std::collections::HashMap;
use nom::lib::std::collections::HashSet;
#[macro_use]
extern crate lazy_static;

#[derive(Eq, PartialEq, Hash, Debug)]
enum PassportDataKey {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryId
}

lazy_static! {
    static ref ALLOWED_EYE_COLORS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("amb");
        s.insert("blu");
        s.insert("brn");
        s.insert("gry");
        s.insert("grn");
        s.insert("hzl");
        s.insert("oth");
        s
    };
}

impl PassportDataKey {
    fn from(name: &str) -> Option<PassportDataKey> {
        match name {
            "byr" => Some(PassportDataKey::BirthYear),
            "iyr" => Some(PassportDataKey::IssueYear),
            "eyr" => Some(PassportDataKey::ExpirationYear),
            "hgt" => Some(PassportDataKey::Height),
            "hcl" => Some(PassportDataKey::HairColor),
            "ecl" => Some(PassportDataKey::EyeColor),
            "pid" => Some(PassportDataKey::PassportID),
            "cid" => Some(PassportDataKey::CountryId),
            _ => None
        }
    }

    fn validate(&self, value: &str) -> bool {
        match self {
            PassportDataKey::BirthYear => value.parse::<i32>().map(|year| (1920 ..= 2002).contains(&year)).unwrap_or(false),
            PassportDataKey::IssueYear => value.parse::<i32>().map(|year| (2010 ..= 2020).contains(&year)).unwrap_or(false),
            PassportDataKey::ExpirationYear => value.parse::<i32>().map(|year| (2020 ..= 2030).contains(&year)).unwrap_or(false),
            PassportDataKey::Height => {
                let valid_range = if value.ends_with("cm") {
                    150 ..= 193
                } else if value.ends_with("in") {
                    59 ..= 76
                } else {
                    return false
                };
                value.chars().take(value.len() - 2).collect::<String>().parse::<i32>().map(|height| valid_range.contains(&height)).unwrap_or(false)
            },
            PassportDataKey::HairColor => value.len() == 7 && value.starts_with('#') && value.chars().skip(1).all(|c| ('0' ..= '9').contains(&c) || ('a' ..= 'f').contains(&c)),
            PassportDataKey::EyeColor => ALLOWED_EYE_COLORS.contains(value),
            PassportDataKey::PassportID => value.len() == 9 && value.chars().all(|c| ('0' ..= '9').contains(&c)),
            PassportDataKey::CountryId => true,
        }
    }
}

struct Passport {
    fields: HashMap<PassportDataKey, String>
}

impl Passport {
    fn parse(line: &str) -> Result<Passport> {
        let mut fields = HashMap::new();
        for field in line.split_whitespace() {
            match field.split(":").collect::<Vec<&str>>()[..2] {
                [name, value] => if let Some(key) = PassportDataKey::from(name) {
                    fields.insert(key, value.to_string());
                } else {
                    bail!("Unparseable line: {}", line);
                },
                _ => bail!("Unparseable line: {}", line),
            }
        }
        Ok(Passport { fields })
    }

    fn valid_for_pt1(&self) -> bool {
        self.fields.len() == 8 || (self.fields.len() == 7 && !self.fields.contains_key(&PassportDataKey::CountryId))
    }

    fn valid_for_pt2(&self) -> bool {
        self.valid_for_pt1() && self.fields.iter().all(|(key, value)| key.validate(value))
    }
}




fn main() -> Result<()> {
    let input = read_input(4)?;
    let mut passports = Vec::new();
    for passport_line in input.split("\n\n") {
        passports.push(Passport::parse(passport_line)?);
    }

    let pt1_valid_count = passports.iter().filter(|p| p.valid_for_pt1()).count();
    println!("There are {} valid passports", pt1_valid_count);

    let pt2_valid_count = passports.iter().filter(|p| p.valid_for_pt2()).count();
    println!("There are {} valid passports round 2", pt2_valid_count);

    Ok(())
}
extern crate lazy_static;
extern crate regex;
use lazy_static::*;
use rand::prelude::*;
use rand::rngs::OsRng;
use regex::Regex;

lazy_static! {
    pub static ref DICE_CMD_PATTERN: Regex = Regex::new(r"\d+(d\d+)?").unwrap();
}

/// Describes a set of dice that can be rolled all at once, i.e. 2d6
#[derive(Eq, PartialEq, Debug)]
pub struct Dice {
    count: u32,
    range: u32,
}

impl Dice {
    pub fn new(count: u32, range: u32) -> Self {
        Self { count, range }
    }

    pub fn gen_result(&self) -> DiceResult {
        DiceResult(
            (0..self.count)
                .map(|_| OsRng.gen_range(0, self.range) + 1)
                .collect(),
        )
    }

    pub fn gen_from_fn<F: FnMut(u32) -> u32>(&self, mut f: F) -> DiceResult {
        DiceResult((0..self.count).map(|_| f(self.range)).collect())
    }
}

impl ::std::str::FromStr for Dice {
    type Err = DiceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !DICE_CMD_PATTERN.is_match(s) {
            return Err(DiceParseError {
                exp: InvalidExpression,
            });
        }
        let split: Vec<_> = s.split('d').filter_map(|p| p.parse().ok()).collect();
        match &split[..] {
            [count, range] => Ok(Dice::new(*count, *range)),
            [range] => Ok(Dice::new(1, *range)),
            _ => Err(DiceParseError {
                exp: InvalidExpression,
            }),
        }
    }
}

pub struct DiceResult(Vec<u32>);

impl DiceResult {
    pub fn new(v: Vec<u32>) -> Self {
        Self(v)
    }

    pub fn total(&self) -> u32 {
        self.0.iter().sum()
    }
}

impl std::fmt::Display for DiceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let as_string: Vec<_> = self.0.iter().map(|v| v.to_string()).collect();
        write!(f, "{} ({})", as_string.join(","), self.total())
    }
}

#[derive(Debug)]
pub struct DiceParseError {
    exp: InvalidExpression,
}

impl ::std::error::Error for DiceParseError {}

impl std::fmt::Display for DiceParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", &self.exp.to_string())
    }
}

#[derive(Debug)]
pub struct InvalidExpression;

impl std::error::Error for InvalidExpression {}

impl std::fmt::Display for InvalidExpression {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

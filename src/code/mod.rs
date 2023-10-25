use paste::paste;
use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
    str::FromStr,
};

use crate::library::io::Scanner;

macro_rules! questions {
    ($($q:ident),*) => {
        $(
            pub mod $q;
        )*
        paste! {
          pub enum Question {
              $([<$q:upper>]),*
          }

          impl Question {
            pub fn run(&self) -> Result<(), Box<dyn Error + 'static>> {
              match self {
                $(
                  Question::[<$q:upper>] => $q::main(),
                )*
              }
            }

            pub fn run_solve<I: BufRead, O: Write>(
              &self,
              input: Scanner<I>,
              output: BufWriter<O>,
            ) -> Result<(), Box<dyn Error + 'static>> {
              match self {
                $(
                  Question::[<$q:upper>] => $q::solve(input, output),
                )*
              }
            }

            pub fn file_path(&self) -> String {
              match self {
                $(
                  Question::[<$q:upper>] => format!("src/code/{}.rs", stringify!($q)),
                )*
              }
            }
          }

          impl FromStr for Question {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
              match s.to_lowercase().as_str() {
                $(
                  stringify!($q) => Ok(Question::[<$q:upper>]),
                )*
                _ => Err(format!("{} is not a valid question", s)),
              }
            }
          }
        }
    };
}

questions! {
  a, b, c
}

pub mod stress;
pub mod template;

pub mod func;
use func::check_func;

use color_eyre::eyre::Result;

use eyre::bail;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use crate::config::cli::{CheckType, get_check_type};

fn get_symbols_info_text(bin_path: &Path) -> Result<String> {
    let output = Command::new("readelf")
        .arg("--syms")
        .arg(bin_path)
        .output()?;
    if !output.status.success() {
        bail!("Failed to run readelf on {:?}", bin_path);
    }

    let output = String::from_utf8_lossy(&output.stdout);
    Ok(output.to_string())
}

#[derive(Debug)]
enum SymType {
    Func,
    File,
    Obj,
    Other(String),
}

impl SymType {
    pub fn new() -> Self {
        Self::Func
    }
}

impl FromStr for SymType {
    type Err = eyre::Report;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "FUNC" => Ok(Self::Func),
            "FILE" => Ok(Self::File),
            "OBJECT" => Ok(Self::Obj),
            _ => Ok(Self::Other(s.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct SymInfo {
    sym_type: SymType,
    ndx: String,
    name: String,
}

impl SymInfo {
    fn check_defined(&self) -> bool {
        let word = &self.ndx;
        let num_res = word.parse::<u32>();
        if num_res.is_err() {
            return false;
        }
        let num = num_res.unwrap();
        num > 0
    }

    fn from_re_rec(rec: &str) -> Result<Self> {
        let mut sym_type = SymType::new();
        let mut ndx = String::new();
        let mut name = String::new();
        for (idx, word) in rec.split_whitespace().enumerate() {
            match idx {
                3 => {
                    sym_type = SymType::from_str(word)?;
                }
                6 => {
                    ndx = word.to_string();
                }
                7 => {
                    name = word.to_string();
                }
                _ => {
                    continue;
                }
            }
        }

        Ok(SymInfo {
            sym_type,
            ndx,
            name,
        })
    }
}

pub fn check() -> Result<()> {
    let check_type = get_check_type();
    match check_type {
        CheckType::Func { func_name } => {
            let info_op = check_func(func_name)?;
            match info_op {
                Some(info) => {
                    println!("Function {} found in the binary", func_name);
                    println!("Symbol info: {:?}", info);
                }
                None => {
                    eprintln!("Function {} not found in the binary", func_name);
                }
            }
        }
    }
    Ok(())
}

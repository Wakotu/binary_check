use std::path::Path;

use color_eyre::eyre::Result;

use crate::check::SymType;

use super::{SymInfo, get_symbols_info_text};
use crate::config::cli::get_bin_path;

fn extract_line(line: &str) -> Result<SymInfo> {
    SymInfo::from_re_rec(line)
}

pub fn check_func_impl(bin_path: &Path, func_name: &str) -> Result<Option<SymInfo>> {
    let text = get_symbols_info_text(bin_path)?;

    // handle output
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("Symbol table") || line.starts_with("Num:") {
            continue;
        }

        let sym_info = extract_line(line)?;
        if func_name == sym_info.name {
            if !sym_info.check_defined() {
                log::warn!(
                    "Symbol {} found, but it is not defined in the binary",
                    func_name
                );
                log::warn!("Symbol type: {:?}", sym_info.sym_type);
                return Ok(None);
            }

            if let SymType::Func = sym_info.sym_type {
                return Ok(Some(sym_info));
            }

            log::warn!("Symbol {} found, but it is not a function", func_name);
            log::warn!("Symbol type: {:?}", sym_info.sym_type);
            return Ok(None);
        }
    }
    Ok(None)
}

pub fn check_func(func_name: &str) -> Result<Option<SymInfo>> {
    let bin_path = get_bin_path();
    check_func_impl(bin_path, func_name)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::utils::report::init_report_utils;

    use super::*;
    use color_eyre::eyre::Result;

    #[test]
    fn test_check_function() -> Result<()> {
        init_report_utils()?;
        let op = check_func_impl(&PathBuf::from_str("./inputs/a.out")?, "say_hello")?;
        assert!(op.is_some());
        Ok(())
    }
}

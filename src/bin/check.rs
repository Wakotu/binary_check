use binary_check::{check::check, utils::report::init_report_utils};
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    init_report_utils()?;
    check()?;
    Ok(())
}

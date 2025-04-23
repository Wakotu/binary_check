pub mod cli {

    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use std::sync::OnceLock;

    use clap::Parser;
    use clap::Subcommand;

    #[derive(Parser)]
    #[command(about, version)]
    pub struct Cli {
        bin_path: String,
        #[command(subcommand)]
        check_type: CheckType,
    }

    #[derive(Subcommand, Debug)]
    pub enum CheckType {
        Func { func_name: String },
    }

    pub fn get_cli() -> &'static Cli {
        static CLI: OnceLock<Cli> = OnceLock::new();
        CLI.get_or_init(Cli::parse)
    }

    pub fn get_check_type() -> &'static CheckType {
        let cli = get_cli();
        &cli.check_type
    }

    pub fn get_bin_path() -> &'static Path {
        static BIN_PATH: OnceLock<PathBuf> = OnceLock::new();
        BIN_PATH.get_or_init(|| {
            let cli = get_cli();
            let bin_str = &cli.bin_path;
            let bin_path_res = PathBuf::from_str(bin_str);
            bin_path_res.unwrap_or_else(|e| {
                panic!("Failed to construct path from input: {}, {e}", bin_str);
            })
        })
    }
}

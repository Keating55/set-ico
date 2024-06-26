use clap::Parser;
use std::path::PathBuf;
mod utils;
use utils::IcoReg;

#[derive(Parser)]
#[command(
    version,
    author,
    about,
    disable_version_flag = true,
    after_help(r#"set-ico -e json -i D:\json.ico -a D:\notepad++.exe"#),
    help_template = "\
{name} {version}
{author}
{about-with-newline}
\x1B[32m{usage-heading}\x1B[0m {usage}

\x1B[32m{all-args}\x1B[0m
{after-help}


"
)]
struct Cli {
    /// file extension e.g. json
    #[arg(short = 'e', long, long_help)]
    extension: String,
    /// ico file absolute path
    #[arg(short = 'i', long)]
    ico: PathBuf,
    /// app file absolute path
    #[arg(short = 'a', long)]
    app: PathBuf,
    /// add `-r` (Administrator) will write in HKLM and HKCU.  default only write in HKCU
    #[arg(short = 'r', long, default_value = "false")]
    root: bool,
}

fn main() {
    let cli = Cli::parse();
    if !(cli.app.is_file() & cli.app.is_absolute() & cli.ico.is_file() & cli.ico.is_absolute()) {
        panic!("Please enter a valid absolute path")
    }
    let icoreg = IcoReg::new(
        cli.extension,
        cli.ico.to_string_lossy().into_owned(),
        cli.app.to_string_lossy().into_owned(),
        cli.root,
        cli.app.file_name().unwrap().to_str().unwrap().to_string(),
    );
    if icoreg.check_extension() {
        panic!("{} is executable file extension", icoreg.extension)
    }
    icoreg.set_file_extision_type();
    icoreg.set_file_extision();
    icoreg.set_create_applications();
    icoreg.set_default_open_app();
    if icoreg.root {
        let icoreguser = IcoReg{root:false,..icoreg};
        icoreguser.set_file_extision_type();
        icoreguser.set_file_extision();
        icoreguser.set_create_applications();
        icoreguser.set_default_open_app();
    }
}

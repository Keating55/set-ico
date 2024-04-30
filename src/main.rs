use clap::Parser;
use set_ico::IcoReg;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version,author,about
    ,after_help(r#"set_ico -e "json" -i "D:\file.ico" -a "D:\notepad++.exe" -p "Applications\notepad++.exe""#)
    ,help_template="\
{name} {version}
{author}
{about-with-newline}
\x1B[32m{usage-heading}\x1B[0m {usage}

\x1B[32m{all-args}\x1B[0m
{after-help}

")]
struct Cli {
    /// file extension e.g. json
    #[arg(short = 'e', long, long_help)]
    extension: String,
    /// ico file path
    #[arg(short = 'i', long)]
    ico: PathBuf,
    /// application path
    #[arg(short = 'a', long)]
    app: PathBuf,
    /// application progid e.g. Applications\notepad++.exe
    #[arg(short = 'p', long, default_value = r#"Applications\notepad++.exe"#)]
    progid: String,
    /// add `-r`` will write in hkcr.  default write in hkcu
    #[arg(short = 'r', long, default_value = "false")]
    root: bool,
}

fn main() {
    let cli = Cli::parse();
    let icoreg = IcoReg::new(
        cli.extension,
        cli.ico.to_string_lossy().into_owned(),
        cli.app.to_string_lossy().into_owned(),
        cli.progid,
        cli.root,
    );
    icoreg.set_file_extision_type();
    icoreg.set_file_extision();
    icoreg.set_default_open_app();
}

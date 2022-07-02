pub use std::env::Args;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

pub use anyhow::{self, bail, ensure, Result};
pub use log::{self, debug, error, info, trace, warn};

pub mod env;
pub mod fs;

mod router;
pub use router::TaskRouter;

mod logger;
use logger::Logger;

pub fn init_logger() {
    Logger::init();
}

pub fn command<S, I>(program: S, args: I) -> Command
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
{
    let mut cmd = Command::new(program);
    cmd.args(args);
    info!(
        "command(program: {:?}, args: {:?})",
        cmd.get_program(),
        cmd.get_args().collect::<Vec<_>>(),
    );
    cmd
}

pub fn run<S, I>(program: S, args: I) -> Result<()>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
{
    let mut cmd = command(program, args);
    cmd.status()?;
    Ok(())
}

pub fn exit(code: i32) -> ! {
    match code {
        0 => info!("exit(code: {})", code),
        _ => error!("exit(code: {})", code),
    }
    std::process::exit(code);
}

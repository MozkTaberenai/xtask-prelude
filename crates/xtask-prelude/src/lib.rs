pub use std::env::Args;
use std::ffi::OsStr;
pub use std::path::{Path, PathBuf};
use std::process::Command;

pub use anyhow::{self, bail, ensure, Result};
pub use log::{self, debug, error, info, trace, warn};

mod cmd;
pub use cmd::cmd;
pub mod env;
pub mod fs;

mod router;
pub use router::TaskRouter;

mod logger;
use logger::Logger;

pub fn init_logger() {
    Logger::init();
}

#[deprecated]
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

#[deprecated]
#[allow(deprecated)]
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
        0 => info!("exit code={code}"),
        _ => error!("exit code={code}"),
    }
    std::process::exit(code);
}

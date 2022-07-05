use std::ffi::OsStr;
use std::process::{Child, Command, ExitStatus, Output};

use super::*;

#[derive(Debug)]
pub struct Error(ExitStatus);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

pub fn cmd<S, I>(args: I) -> Cmd
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
{
    let mut args = args.into_iter();
    let program = match args.next() {
        None => {
            error!("no program specified");
            exit(1);
        }
        Some(s) => s,
    };

    let mut inner = Command::new(program);
    inner.args(args);
    Cmd { inner }
}

pub struct Cmd {
    inner: std::process::Command,
}

impl Cmd {
    pub fn current_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.inner.current_dir(dir);
        self
    }

    pub fn env(mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> Self {
        self.inner.env(key, val);
        self
    }

    pub fn env_clear(mut self) -> Self {
        self.inner.env_clear();
        self
    }

    pub fn env_remove(mut self, key: impl AsRef<OsStr>) -> Self {
        self.inner.env_remove(key);
        self
    }

    fn log(&self, label: &str) {
        let program = self.inner.get_program();
        let args = self.inner.get_args().collect::<Vec<_>>();

        let current_dir_phrase = match self.inner.get_current_dir() {
            None => "".to_owned(),
            Some(path) => format!(" current_dir={:?}", path),
        };

        let envs = self.inner.get_envs();
        let envs_phrase = match envs.len() {
            0 => "".to_owned(),
            _ => format!(" envs={:?}", envs.collect::<Vec<_>>()),
        };

        info!("{label}: program={program:?} args={args:?}{current_dir_phrase}{envs_phrase}",);
    }

    pub fn run(mut self) -> Result<()> {
        self.log("run");
        let status = self.inner.status()?;
        match status.success() {
            true => Ok(()),
            false => Err(Error(status))?,
        }
    }

    pub fn status(mut self) -> Result<ExitStatus> {
        self.log("status");
        Ok(self.inner.status()?)
    }

    pub fn output(mut self) -> Result<Output> {
        self.log("output");
        Ok(self.inner.output()?)
    }

    pub fn spawn(mut self) -> Result<Child> {
        self.log("spawn");
        Ok(self.inner.spawn()?)
    }
}

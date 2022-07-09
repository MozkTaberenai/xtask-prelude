use super::*;

pub fn a(args: Args) -> Result<()> {
    info!("{args:?}");
    Ok(())
}

pub fn b(args: Args) -> Result<()> {
    info!("{args:?}");
    bail!("fail!!");
}

pub fn c(args: Args) -> Result<()> {
    error!("{args:?}");
    Ok(())
}

pub fn run_echo(args: Args) -> Result<()> {
    let args = ["echo".to_string()].into_iter().chain(args);
    cmd(args).run()?;
    Ok(())
}

pub fn run_echo_quiet(args: Args) -> Result<()> {
    let args = ["echo".to_string()].into_iter().chain(args);
    cmd(args).quiet().run()?;
    Ok(())
}

pub fn run_with_env(_args: Args) -> Result<()> {
    cmd(["echo", "ok"]).env("AAA", "aaa").run()?;
    Ok(())
}

pub fn run_with_chdir(_args: Args) -> Result<()> {
    cmd(["ls"]).current_dir("xtask").run()?;
    Ok(())
}

pub fn run_unknown_command(_args: Args) -> Result<()> {
    cmd(["a"]).run()?;
    Ok(())
}

pub fn run_fail_command(_args: Args) -> Result<()> {
    cmd(["sh", "-c", "exit 1"]).run()?;
    Ok(())
}

pub fn dir(_args: Args) -> Result<()> {
    fs::create_dir_all("a/b/c")?;
    fs::remove_dir_all("a")?;
    Ok(())
}

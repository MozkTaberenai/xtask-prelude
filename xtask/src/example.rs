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
    info!("{args:?}");
    Ok(())
}

pub fn run_echo(args: Args) -> Result<()> {
    let args = args.collect::<Vec<_>>();
    run("echo", args.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;
    Ok(())
}

pub fn dir(_args: Args) -> Result<()> {
    fs::create_dir_all("a/b/c")?;
    fs::remove_dir_all("a")?;
    Ok(())
}

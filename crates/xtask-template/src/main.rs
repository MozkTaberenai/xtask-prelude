use xtask_prelude::*;

fn main() {
    init_logger();
    TaskRouter::new().insert("fmt", fmt).run();
}

fn fmt(_args: Args) -> Result<()> {
    run("cargo", ["fmt"])?;
    Ok(())
}

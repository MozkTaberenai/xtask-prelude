use xtask_prelude::*;

mod example;

fn main() {
    init_logger();
    TaskRouter::new()
        .insert("fmt", fmt)
        .insert("lint", lint)
        .insert("example-a", example::a)
        .insert("example-b", example::b)
        .insert("example-c", example::c)
        .insert("example-run-echo", example::run_echo)
        .insert("example-dir", example::dir)
        .run();
}

fn fmt(_args: Args) -> Result<()> {
    run("cargo", ["fmt"])?;
    run("taplo", ["format", "**/*.toml"])?;
    Ok(())
}

fn lint(_args: Args) -> Result<()> {
    run("cargo", ["check"])?;
    run("cargo", ["clippy"])?;
    Ok(())
}

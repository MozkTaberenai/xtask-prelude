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
        .insert("example-run-echo-quiet", example::run_echo_quiet)
        .insert("example-run-with-env", example::run_with_env)
        .insert("example-run-with-chdir", example::run_with_chdir)
        .insert("example-run-unknown-command", example::run_unknown_command)
        .insert("example-run-fail-command", example::run_fail_command)
        .insert("example-dir", example::dir)
        .run();
}

fn fmt(_args: Args) -> Result<()> {
    cmd(["cargo", "fmt"]).run()?;
    cmd(["taplo", "format", "**/*.toml"]).run()?;
    Ok(())
}

fn lint(_args: Args) -> Result<()> {
    cmd(["cargo", "check"]).run()?;
    cmd(["cargo", "clippy"]).run()?;
    Ok(())
}

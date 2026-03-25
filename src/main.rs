use anyhow::{Context, Result};
use fakeroot_rs::FakerootCommandExt;
use fakeroot_rs::loginshell;
use nix::unistd::getuid;
use std::env;
use std::process::Command;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let (cmd_name, cmd_args) = if let Some(name) = args.next() {
        (name, args.collect::<Vec<_>>())
    } else {
        let uid = getuid().as_raw();
        let shell = loginshell::get_login_shell(uid).context("Failed to get login shell")?;
        (shell, vec![])
    };

    let mut cmd = Command::new(&cmd_name);
    let status = cmd
        .args(&cmd_args)
        .fakeroot()?
        .status()
        .with_context(|| format!("Failed to execute command: {}", cmd_name))?;

    if let Some(code) = status.code() {
        std::process::exit(code);
    } else {
        eprintln!("Command terminated by signal");
        std::process::exit(1);
    }
}

#![doc = include_str!("../README.md")]

use anyhow::Result;
use nix::sched::{CloneFlags, unshare};
use nix::unistd::{getgid, getuid};
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{fs, io};

pub mod loginshell;

pub trait FakerootCommandExt {
    fn fakeroot(&mut self) -> Result<&mut Self>;
}

impl FakerootCommandExt for Command {
    fn fakeroot(&mut self) -> Result<&mut Self> {
        let uid = getuid();
        if uid.is_root() {
            return Ok(self);
        }
        let gid = getgid();

        unsafe {
            self.pre_exec(move || {
                unshare(CloneFlags::CLONE_NEWUSER).map_err(io::Error::other)?;

                fs::write("/proc/self/setgroups", "deny").map_err(io::Error::other)?;

                let gid_map = format!("0 {} 1", gid);
                fs::write("/proc/self/gid_map", gid_map).map_err(io::Error::other)?;

                let uid_map = format!("0 {} 1", uid);
                fs::write("/proc/self/uid_map", uid_map).map_err(io::Error::other)?;

                Ok(())
            });
        }
        Ok(self)
    }
}

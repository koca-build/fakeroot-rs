use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_login_shell(uid: u32) -> Result<String> {
    let passwd_path = Path::new("/etc/passwd");
    let file = File::open(passwd_path).context("Failed to open /etc/passwd")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 7
            && let Ok(luid) = parts[2].parse::<u32>()
            && luid == uid
        {
            return Ok(parts[6].to_string());
        }
    }

    anyhow::bail!("User UID {} not found in /etc/passwd", uid)
}

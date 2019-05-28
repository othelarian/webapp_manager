// PREPARATION -> VALIDATE THE INSTALLATION OF ELM AND UGLIFY

use std::process::Command;
use std::str;

pub enum PrepStatus {
    None(&'static str),
    Installed,
    Optimized
}

impl PrepStatus {
    pub fn check() -> PrepStatus {
        let (res0, res1) = if cfg!(target_os = "windows") {
            (
                Command::new("cmd").args(&["/C", "elm --version"]).output(),
                Command::new("cmd").args(&["/C", "uglify --version"]).output()
            )
        } else {
            (
                Command::new("elm").arg("--version").output(),
                Command::new("uglify").arg("--version").output()
            )
        };
        match res0 {
            Ok(output1) => {
                match str::from_utf8(&output1.stdout).expect("").trim() {
                    "0.19.0" => {
                        match res1 {
                            Ok(_) => PrepStatus::Optimized,
                            Err(_) => PrepStatus::Installed
                        }
                    }
                    &_ => PrepStatus::None("ERROR ==> The version of elm installed is not 0.19.0!")
                }
            }
            Err(_) => PrepStatus::None("ERROR ==> elm executable not installed!")
        }
    }
}

// HANDLE GUI LOGIC

// HANDLE COMPILE LOGIC

// HANDLE SERVE LOGIC
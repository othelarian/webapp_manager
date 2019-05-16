extern crate web_view;

use std::str;
use std::process::Command;
use web_view::*;

mod wbam_args;

enum PrepStatus {
    None(bool),
    Installed,
    Optimized
}

impl PrepStatus {
    fn check() -> PrepStatus {
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
                    &_ => PrepStatus::None(true)
                }
            }
            Err(_) => PrepStatus::None(false)
        }
    }

    fn shell_err_message(&self) -> bool {
        match self {
            PrepStatus::None(b) => {
                println!("ERROR ==> elm executable not installed!");
                if b == &true { println!("The version of elm installed is not 0.19.0"); };
                false
            },
            _ => true
        }
    }
}

fn main() {
    // check if external executable are installed
    let prep_status = PrepStatus::check();
    // parse the args
    match wbam_args::get_args() {
        wbam_args::ArgChoice::Gui => {
            //
            // TODO : add all the comm system
            //
            web_view::builder()
                .title("WebApp Manager")
                .content(Content::Html(include_str!("../front/index.html")))
                .size(300, 400)
                .resizable(true)
                .debug(true)
                .user_data(())
                .invoke_handler(|_webview, _arg| Ok(()))
                .run()
                .unwrap();
            //
        }
        wbam_args::ArgChoice::Serve => {
            if !prep_status.shell_err_message() { return; }
            //
            //
        }
        wbam_args::ArgChoice::Compile => {
            if !prep_status.shell_err_message() { return; }
            //
            //
        }
    }
    //
    // TODO : if "compile", only execute the compile part
    //
    // TODO : if "compile", check for the "optimized", if we do, go for uglify (with a check)
    //
    // TODO : if "headless", check if there is an arg for the path, otherwise ask for it
    //
    // TODO : if "headless", check if it's only the reactor, or if we need output
}

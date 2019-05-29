// PREPARATION -> VALIDATE THE INSTALLATION OF ELM AND UGLIFY

pub mod wbam_args;

pub mod prep_mode {
    use std::process::Command;
    use std::str;

    pub enum PrepStatus {
        None(&'static str),
        Installed,
        Optimized
    }

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

// HANDLE COMPILE LOGIC

pub mod compile_mode {
    use crate::prep_mode::PrepStatus;
    use crate::wbam_args;
    use std::io::{self, Write};
    use std::path::Path;
    use std::process::Command;

    pub fn compile(prep_status: &PrepStatus, args: &wbam_args::CompileArgs) -> Result<(),&'static str> {
        println!("starting compilation!");
        match Path::new(args.get_file()).exists() {
            true => {
                let mut command = if cfg!(target_os = "windows") { Command::new("cmd") } else { Command::new("sh") };
                if cfg!(target_os = "windows") { command.arg("/C"); } else { command.arg("-c"); }
                command.arg("elm").arg("make").arg(args.get_file());
                if args.is_optimize() { command.arg("--optimize"); }
                match args.get_output() {
                    Some(o) => { command.arg(format!("--output={}",o)); }
                    None => {}
                };
                //
                //
                let output = command.output().expect("ERROR => Failed to launch elm command to compile");
                //
                println!("status: {}",output.status);
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
                //
                // TODO : launch the command (NOT WORKING)
                // TODO : capture the error from elm compiler
                //
                if args.is_optimize() {
                    match prep_status {
                        PrepStatus::None(_) => Ok(()),
                        PrepStatus::Installed => Err("WARNING => No uglify, so no full optimization!"),
                        PrepStatus::Optimized => {
                            //
                            // TODO
                            //
                            Ok(())
                            //
                        }
                    }
                } else { Ok(()) }
            }
            false => Err("ERROR => Failed to parse the provided file as an existing path")
        }
    }
}

// HANDLE SERVE LOGIC

pub mod serve_mode {
    use crate::prep_mode::PrepStatus;
    use crate::wbam_args;

    pub fn serve(prep_status: &PrepStatus, args: &wbam_args::ServeArgs) -> Result<(),&'static str> {
        println!("Starting the service!");
        //
        //
        //
        Ok(())
    }
}

// HANDLE GUI LOGIC

pub mod gui_mode {
    use crate::prep_mode::PrepStatus;
    use crate::wbam_args;
    use web_view::*;


    pub fn gui(prep_status: &PrepStatus, args: &wbam_args::GuiArgs) -> Result<(),&'static str> {
        //
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
        //
        Ok(())
    }
}

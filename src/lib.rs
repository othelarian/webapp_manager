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
                Command::new("cmd").args(&["/C", "uglifyjs --version"]).output()
            )
        } else {
            (
                Command::new("elm").arg("--version").output(),
                Command::new("uglifyjs").arg("--version").output()
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
    use std::error::Error;
    use std::io::{self, Write};
    use std::path::Path;
    use std::process::{Command,Stdio};

    pub fn compile(prep_status: &PrepStatus, args: &wbam_args::CompileArgs) -> Result<(),&'static str> {
        println!("starting compilation!");
        match Path::new(args.get_file()).exists() {
            true => {
                /*
                let mut command = if cfg!(target_os = "windows") { Command::new("cmd") } else { Command::new("sh") };
                if cfg!(target_os = "windows") { command.arg("/C"); } else { command.arg("-c"); }
                command.arg("elm make").arg(args.get_file());
                if args.is_optimize() { command.arg("--optimize"); }
                match args.get_output() {
                    Some(o) => { command.arg(format!("--output={}",o)); }
                    None => {}
                };
                */
                //
                // TODO : the code below work on linux, but need to be tested on windows
                //
                let mut command = Command::new("elm");
                command.arg("make");
                command.arg(args.get_file());
                if args.is_optimize() { command.arg("--optimize"); }
                match args.get_output() {
                    Some(o) => { command.arg(format!("--output={}",o)); }
                    None => {}
                };
                //
                let output = command.output().expect("ERROR => Failed to launch elm command to compile");
                println!("status: {}",output.status);
                //
                // TODO : check if there is an error in stderr
                //
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
                if args.is_optimize() {
                    // check if the output can be minified
                    match args.get_output() {
                        None => Err("WARNING => No output specifed, so no optimization!"),
                        Some(o) => {
                            if &o[((&o.len())-3)..] != ".js" {
                                return Err("WARNING => No optimization, this is not js as output!")
                            }
                            // check if uglify is ok, and finish the compilation
                            match prep_status {
                                PrepStatus::None(_) => Ok(()),
                                PrepStatus::Installed => Err("WARNING => No uglify, so no full optimization!"),
                                PrepStatus::Optimized => {
                                    //
                                    // TODO : like the elm command, work with linux, not tested with windows
                                    //
                                    let mut uglify1 = Command::new("uglifyjs");
                                    uglify1.arg(o);
                                    uglify1.arg("--compress");
                                    uglify1.arg("pure_funcs=\"F2,F3,F4,F5,F6,F7,F8,F9,A2,A3,A4,A5,A6,A7,A8,A9\",pure_getters,keep_fargs=false,unsafe_comps,unsafe");
                                    uglify1.stdout(Stdio::piped());
                                    match uglify1.spawn() {
                                        Err(_) => Err("OPTIMIZE PANIC => Error with the first uglifyjs"),
                                        Ok(out1) => {
                                            let minname = String::from(&o[..(&o.len()-3)])+".min.js";
                                            let mut uglify2 = Command::new("uglifyjs");
                                            uglify2.arg("--mangle")
                                                .arg(String::from("--output=")+&minname)
                                                .stdin(out1.stdout.unwrap());
                                            match uglify2.output() {
                                                Err(_) => Err("OPTIMIZE PANIC => Error with the second uglifyjs"),
                                                Ok(_) => Ok(())
                                            }
                                        }
                                    }
                                }
                            }
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

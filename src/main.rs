extern crate clap;

use clap::App;
use std::process::Command;

//
// TODO : launch two threads / processus, for the elm reactor, and for the make
// TODO : launch the webview, if needed
//

fn main() {
    //
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
    //
    // TODO : check if elm and uglify are installed
    //
    // TODO : check for the args
    //
    App::new("test app")
        .version("1.0")
        .about("This app is for...")
        .author("othelarian")
        .get_matches();
    //
    // TODO : if "compile", only execute the compile part
    //
    // TODO : if "compile", check for the "optimized", if we do, go for uglify (with a check)
    //
    // TODO : if "headless", check if there is an arg for the path, otherwise ask for it
    //
    // TODO : if "headless", check if it's only the reactor, or if we need output
    //
    //println!("Hello, world!");
}

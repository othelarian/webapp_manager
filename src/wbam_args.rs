extern crate clap;

use clap::{Arg,App,SubCommand};
use std::path::Path;

pub enum ArgChoice {
    Gui,
    Compile,
    Serve
}

impl ArgChoice {
    fn create_compile(path: String) -> ArgChoice {
        //
        //
        ArgChoice::Compile
        //
    }
}

pub fn get_args() -> ArgChoice {
    let matches = App::new("webapp manager")
        .version("1.0")
        .about("A both visual and shell app to manage the status of my elm app")
        .author("othelarian Inc.")
        //
        .arg(arg_dir())
        //
        .subcommand(go_serve())
        .subcommand(go_compile())
        //
        .get_matches();
    //
    // TODO : manage the subcommands
    //
    if let Some(matches) = matches.subcommand_matches("compile") { // compile mode
        //
        //
        ArgChoice::Compile
        //
    } else if let Some(matches) = matches.subcommand_matches("serve") {
        //
        ArgChoice::Serve
        //
    } else {
        //
        ArgChoice::Gui
        //
    }
    //
    /*if matches.is_present("compile") {
        //
        ArgChoice::Compile
        //
    }*/
    //
}

fn arg_dir<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("DIRECTORY")
        .help("the directory of your elm project")
}

fn go_serve<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("serve")
        .about("Serve your project with automatic rebuild")
        //
        // TODO : take care of the no reactor
        // TODO : (optional) specify if it's verbose or not
        //
        .arg(Arg::with_name("no reactor")
            .short("n")
            .long("noreactor")
            .help("only rebuild the files, without serving with the reactor"))
        //
        .arg(arg_dir())
        //
        // TODO : add FILES args
        //
}

fn go_compile<'a,'b>() -> App<'a,'b> {
    //
    // TODO : define what 'compile' need
    //
    SubCommand::with_name("compile")
        .about("Compile your project instead of served it or use GUI")
        .arg(Arg::with_name("FILE")
            .required(true)
            .help("the name of the file we want to compile"))
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .long("output")
            .help("specify the final destination of your compilation"))
        //
        // TODO : add the FILE required arg
        // TODO : add the OUTPUT optional arg
        // TODO : add an --optimized arg
        //
    //
}

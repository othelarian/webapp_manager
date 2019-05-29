extern crate clap;

use clap::{Arg,ArgMatches,App,SubCommand};

pub struct CompileArgs {
    file: String,
    optimize: bool,
    output: Option<String>
}

impl CompileArgs {
    pub fn get_file(&self) -> &String { &self.file }
    pub fn is_optimize(&self) -> bool { self.optimize }
    pub fn get_output(&self) -> &Option<String> { &self.output }
}

pub struct ServeArgs {
    //
    //
}

pub struct GuiArgs {
    directory: Option<String>
}

pub enum ArgChoice {
    Compile(CompileArgs),
    Serve(ServeArgs),
    Gui(GuiArgs)
}

impl ArgChoice {
    fn create_compile<'a>(args: &ArgMatches<'a>) -> ArgChoice {
        ArgChoice::Compile(CompileArgs {
            file: String::from(args.value_of("FILE").unwrap()),
            optimize: args.is_present("optimize"),
            output: match args.value_of("OUTPUT") {
                Some(v) => Some(v.to_string()),
                None => None
            }
        })
    }

    fn create_serve<'a>(args: &ArgMatches<'a>) -> ArgChoice {
        //
        //
        ArgChoice::Serve(ServeArgs {
            //
            //
        })
        //
    }
    fn create_gui<'a>(args: ArgMatches<'a>) -> ArgChoice {
        ArgChoice::Gui(GuiArgs {
            directory: match args.value_of("DIRECTORY") {
                Some(v) => Some(v.to_string()),
                None => None
            }
        })
    }
}

pub fn get_args() -> ArgChoice {
    let matches = App::new("webapp manager")
        .version("1.0")
        .about("A both visual and shell app to manage the status of my elm app")
        .author("othelarian Inc.")
        .arg(arg_dir())
        .subcommand(go_serve())
        .subcommand(go_compile())
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("compile") {
        ArgChoice::create_compile(matches)
    } else if let Some(matches) = matches.subcommand_matches("serve") {
        ArgChoice::create_serve(matches)
    } else {
        ArgChoice::create_gui(matches)
    }
}

fn arg_dir<'a,'b>() -> Arg<'a,'b> {
    Arg::with_name("DIRECTORY")
        .help("the directory of your elm project")
}

fn go_serve<'a,'b>() -> App<'a,'b> {
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
    SubCommand::with_name("compile")
        .about("Compile your project instead of served it or use GUI")
        .arg(Arg::with_name("FILE")
            .required(true)
            .help("the name of the file we want to compile"))
        .arg(Arg::with_name("OUTPUT")
            .short("u")
            .long("output")
            .value_name("PATH")
            .help("specify the final destination of your compilation"))
        .arg(Arg::with_name("optimize")
            .short("o")
            .long("optimize")
            .help("specify if we want to compile with uglify (if installed)"))
}

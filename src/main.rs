extern crate web_view;

use std::process;

use webapp_manager::prep_mode;
use webapp_manager::wbam_args;

//mod wbam_args;

fn main() {
    // check if external executable are installed
    let prep_status = prep_mode::check();
    let kill_app = |prep: &prep_mode::PrepStatus| {
        if let prep_mode::PrepStatus::None(e) = prep {
            println!("{}", e);
            process::exit(1);
        }
    };
    // parse the args and run
    let res = match wbam_args::get_args() {
        wbam_args::ArgChoice::Gui(args) => {
            use webapp_manager::gui_mode;
            gui_mode::gui(&prep_status, &args)
        }
        wbam_args::ArgChoice::Serve(args) => {
            kill_app(&prep_status);
            use webapp_manager::serve_mode;
            serve_mode::serve(&prep_status, &args)
        }
        wbam_args::ArgChoice::Compile(args) => {
            kill_app(&prep_status);
            use webapp_manager::compile_mode;
            compile_mode::compile(&prep_status, &args)
        }
    };
    // finish the program
    match res {
        Ok(_) => (),
        Err(e) => println!("QUIT WITH MESSAGE:\n{}",e)
    }
}

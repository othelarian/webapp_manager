extern crate web_view;

use std::process;
use web_view::*;

use webapp_manager::PrepStatus;

mod wbam_args;

fn main() {
    // check if external executable are installed
    let prep_status = PrepStatus::check();
    //
    // TODO : closure for the prep uninstalled case
    //
    // parse the args and run
    match wbam_args::get_args() {
        wbam_args::ArgChoice::Gui => {
            //
            // TODO : move all the logic in the lib !!!!!
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
        wbam_args::ArgChoice::Fail => {
            //
            //
        }
        wbam_args::ArgChoice::Serve => {
            if let PrepStatus::None(e) = prep_status { /* TODO */ }
            //
            //
            println!("Starting the service!");
            //
            // TODO : start serving
            //
        }
        wbam_args::ArgChoice::Compile => {
            if let PrepStatus::None(e) = prep_status { /* TODO */ }
            //
            //
            println!("starting compilation!");
            //
            // TODO : start compiling
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
    //
    if let Err(e) = run(prep_status) {
        println!("{}", e);
        process::exit(1);
    }
}

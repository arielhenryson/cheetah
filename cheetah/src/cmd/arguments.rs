use clap::{ Arg, App, ArgMatches };

// add basic information about the app
// and set the app argument
pub fn app_arguments<'a>() -> ArgMatches<'a> {
    App::new("Cheetah")
        .version("0.1.0")
        .author("Ariel Henryson <ariel.henryson@gmail.com>")
        .about("Cheetah engine")
        .arg(Arg::with_name("FILE")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("file name"))
        .get_matches()
}

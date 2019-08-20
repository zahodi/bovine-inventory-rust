extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("bovine-inventory")
                       .arg(Arg::with_name("debug")
                                   .help("turn on debugging information")
                                   .short("d"))

                       .args(&[
                           Arg::with_name("config")
                                   .help("sets the config file to use")
                                   .takes_value(true)
                                   .short("c")
                                   .long("config"),
                           Arg::with_name("input")
                                   .help("the input file to use")
                                   .index(1)
                                   .required(true)
                       ])

                       .arg_from_usage("--license 'display the license file'")

                       .args_from_usage("[output] 'Supply an output file to use'
                                         -i, --int=[IFACE] 'Set an interface to use'")
                       .get_matches();

}

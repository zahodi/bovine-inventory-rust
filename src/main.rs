extern crate clap;
extern crate serde_yaml;
use clap::{App, Arg};
use std::io;
use std::env;
use std::path::Path;
use std::fs;
mod yaml_parse;

fn main() {
    visit_dirs();
    read_yaml();
}

fn read_yaml() {
    let data = fs::read_to_string("./test_directory/main.yml").expect("Unable to read file");
    println!("{}", data);
}

fn visit_dirs() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

#[allow(dead_code)]
fn cli() {
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

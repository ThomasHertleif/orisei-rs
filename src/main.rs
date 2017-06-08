extern crate clap;
extern crate orisei;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Orisei")
                        .version("0.1")
                        .author("Thomas Hertleif <thomas@hertleif.de>")
                        .about("Batch file renamer")
                        .arg(Arg::with_name("replace")
                            .short("r")
                            .long("replace")
                            .value_name("pattern")
                            .help("e.g.: s/foo/bar will serach for foo and replace with bar")
                            .takes_value(true))
                        .arg(Arg::with_name("prefix")
                            .short("p")
                            .long("prefix")
                            .value_name("prefix")
                            .help("Can also contain a pattern e.g.: `Foo-{count:4}-` will rename `test.jpg` to `Foo-0001-test.jpg`"))
                        .arg(Arg::with_name("suffix")
                            .short("s")
                            .long("suffix")
                            .value_name("suffix")
                            .help("Can also contain a pattern e.g.: `-foo-{count:4}` will rename `test.jpg` to `test-foo-0001.jpg`"))
                        .arg(Arg::with_name("INPUT")
                            .help("Set input files or folder")
                            .required(true)
                            // .last(true)
                            )
                        .get_matches();

    if let Some(input) = matches.value_of("INPUT") {
        println!("Value for output: {}", input);
        orisei::read_dir(input);
    }

    if let Some(replace) = matches.value_of("replace") {
        println!("Replace: {}", replace);
    }

    if let Some(suffix) = matches.value_of("suffix") {
        println!("Prefix: {}", suffix);
    }

    if let Some(prefix) = matches.value_of("prefix") {
        println!("Prefix: {}", prefix);
    }
}

extern crate easy_reader;
extern crate getopts;
extern crate rand;

use std::env;
use std::io::{Cursor, Error, Seek, SeekFrom, Write};

use easy_reader::EasyReader;
use getopts::Options;

fn spook() -> Result<(), Error> {
    
    // Create fake "file"
    let mut c = Cursor::new(Vec::new());
    let str42 = include_str!("spook.lines").as_bytes();

    // Write into the "file" and seek to the beginning
    c.write_all(str42).unwrap();
    c.seek(SeekFrom::Start(0)).unwrap();

    let mut reader = EasyReader::new(c)?;
    let _res = reader.build_index();
    print!("{}", reader.random_line()?.unwrap());
    print!("{}", reader.random_line()?.unwrap());
    print!("{}", reader.random_line()?.unwrap());
    print!("{}", reader.random_line()?.unwrap());
    println!("{}", reader.random_line()?.unwrap());
    Ok(())
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {}", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    let brief = format!(
        "Version: {}",
        "$Id$"
            .replace("$Id: ", "")
            .replace(" $", "")
    );
    print!("{}", &brief);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the version id");
    opts.optflag("c", "clean", "print nothing but the objid");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("v") {
        print_version();
        return;
    }
    let _res = spook();
}

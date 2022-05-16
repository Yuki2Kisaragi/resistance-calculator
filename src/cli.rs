use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn io_open() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    let num_args = argv.len();
    println!("args:{num_args}");
    println!("{:?}", argv);

    let file_name = &argv[1];

    if num_args < 2 {
        println!("insert filename");
        std::process::exit(1);
    }

    let f = File::open(file_name)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

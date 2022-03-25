use std::{env, process};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use scan_dir::{ScanDir};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        println!("Searching directory '{}', for extension of '{}' ", &args[1], &args[2])
    } else {
        help();
        process::exit(0)
    }
    let folder = &args[1].as_str();
    let extension = &args[2].as_str();
    let mut count: u32 = 0;

    let all_rs_files: Vec<_> = ScanDir::files().walk(folder, |iter| {
        iter.filter(|&(_, ref name)| name.ends_with(extension))
            .map(|(ref entry, _)| entry.path())
            .collect()
    }).unwrap();
    for files in all_rs_files {
        if let Ok(lines) = read_lines(files) {
            for line in lines {
                if let Ok(_ip) = line {
                    count +=  1
                }
            }
        }
    }
    println!("all files linecount is: {} ", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn help() {
    println!("Opps, seems like wrong or missing arguments. Make sure you provide path and extension separated by spaces ");
}


use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use scan_dir::ScanDir;

fn main() {
    let mut count: u32 = 0;

    let all_rs_files: Vec<_> = ScanDir::files().walk(".", |iter| {
        iter.filter(|&(_, ref name)| name.ends_with(".rs"))
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

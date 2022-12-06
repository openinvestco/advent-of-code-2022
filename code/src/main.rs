use std::fs::File;
use std::io::{self, BufRead};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
const COPY_DIR: &'static str = "data";

/// A helper function for recursively copying a directory.
fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to = to.as_ref().to_path_buf();
    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(&path, to);
        } else { /* Skip other content */
        }
    }
}

fn build_script() {
    // Request the output directory
    let out = PathBuf::from(format!("target/debug/{}", COPY_DIR));

    // If it is already in the output directory, delete it and start over
    if out.exists() {
        fs::remove_dir_all(&out).unwrap();
    }

    // Create the out directory
    fs::create_dir(&out).unwrap();

    // Copy the directory

    let pwd = env::current_dir().unwrap();
    let dataDir: String = format!("{}/{}", pwd.to_str().unwrap(), COPY_DIR);
    copy_dir(dataDir, &out);
}

fn main() {
    build_script();
    //day1::day1::run();
    //day2::day2::part_1::run();
    //day2::day2::part_2::run();
    //day3::day3::part_1::run();
    //day3::day3::part_2::run();
    //day4::day4::part_2::run();
    day5::day5::part_1::run();
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
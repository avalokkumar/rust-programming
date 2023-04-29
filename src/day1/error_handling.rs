use std::env;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;
use std::fs;
use std::io::{BufRead, BufReader};

fn read_file_using_fs(filename: &str) -> Result<String, Error> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}

fn read_file_using_bufreader(filename: &str) -> Result<String, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut contents = String::new();
    for line in reader.lines() {
        contents.push_str(&line?);
        contents.push('\n');
    }
    Ok(contents)
}

fn read_file_using_readline(filename: &str) -> Result<String, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut contents = String::new();
    for line in reader.lines() {
        let line = line?;
        contents.push_str(&line);
        contents.push('\n');
    }
    Ok(contents)
}

fn read_file(filename: &str) -> Result<String, Error> {
    // Attempt to open the file and handle any errors
    let mut file = match File::open(filename) {
        Ok(f) => f, // If the file is successfully opened, return the file handle
        Err(e) => return Err(e), // If there is an error opening the file, return the error
    };

    // Read the contents of the file into a string and handle any errors
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents), // If the contents are successfully read into the string, return the contents
        Err(e) => Err(e), // If there is an error reading the file, return the error
    }
}


pub fn execute_error_handling() {
    // Attempt to read a file
    println!("Reading File contents using read_file");
    let result = read_file("src/day1/data/file.txt");

    // Handle the result
    match result {
        Ok(contents) => println!("File contents: \n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }


    // Attempt to read a file using read_file_using_fs
    println!("Reading File contents using read_file_using_fs");
    let result = read_file_using_fs("src/day1/data/file.txt");

    // Handle the result
    match result {
        Ok(contents) => println!("File contents: \n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }


    // Attempt to read a file using read_file_using_bufreader
    println!("Reading File contents using read_file_using_bufreader");
    let result = read_file_using_bufreader("src/day1/data/file.txt");

    // Handle the result
    match result {
        Ok(contents) => println!("File contents: \n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Attempt to read a file using read_file_using_readline
    println!("Reading File contents using read_file_using_readline");
    let result = read_file_using_readline("src/day1/data/file.txt");

    // Handle the result
    match result {
        Ok(contents) => println!("File contents: \n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}


/*fn read_file2(file_path: &str) -> std::io::Result<String> {
    let path = PathBuf::from(file_path);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn execute_error_handling2() {
    // Attempt to read a file
    let mut path = env::current_dir().unwrap();
    path.push("src");
    path.push("day1");
    path.push("data");
    path.push("file.txt");

    let result = read_file(&path.to_string_lossy());

    // Handle the result
    match result {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}*/
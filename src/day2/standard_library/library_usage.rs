use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::thread;

pub fn execute_library_usage() {
    // Read from a file
    let lines = read_from_file();

    // Write to a file
    write_to_file(&lines);

    // Work with collections
    word_count(&lines);

    // Make an HTTP request
    // make_http_request();

    // Spawn a thread
    spawn_thread();

    check_collection()

}

fn read_from_file() -> Vec<String> {
    let file = File::open("src/day2/standard_library/data/sample.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    println!("Read {} lines from file", lines.len());
    lines
}

fn write_to_file(lines: &[String]) {
    let mut output_file = File::create("src/day2/standard_library/data/sample_output.txt").expect("Failed to create file");
    for line in lines {
        writeln!(output_file, "{}", line).expect("Failed to write line to file");
    }
    println!("Wrote {} lines to file", lines.len());
}

fn word_count(lines: &Vec<String>) {
    let mut word_counts = HashMap::new();
    for line in lines {
        for word in line.split_whitespace() {
            *word_counts.entry(word.to_owned()).or_insert(0) += 1;
        }
    }
    println!("Found {} unique words", word_counts.len());
}

fn make_http_request() {
    let mut stream = TcpStream::connect("www.example.com:80").unwrap();
    let request = b"GET / HTTP/1.1\r\nHost: www.example.com\r\n\r\n";
    stream.write(request).unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).unwrap();
    let body = from_utf8(&response).unwrap();
    println!("Received response: {}", body);
}

fn spawn_thread() {
    let handle = thread::Builder::new()
        .name("child".to_string())
        .spawn(|| {
            for i in 1..=5 {
                println!("Thread {}: {}", thread::current().name().unwrap(), i);
                thread::sleep(std::time::Duration::from_millis(500));
            }
        })
        .unwrap();
    handle.join().unwrap();
}


fn check_collection() {
    // create a new empty vector
    let mut vec: Vec<i32> = Vec::new();

    // add elements to the vector
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // iterate over the elements in the vector
    for i in &vec {
        println!("{}", i);
    }

    // create a new empty hash map
    let mut map = HashMap::new();

    // add key-value pairs to the hash map
    map.insert("apple", 3);
    map.insert("banana", 2);
    map.insert("orange", 5);

    // get the value associated with a key
    let apple_count = map.get("apple");
    println!("apple count: {:?}", apple_count);

    // iterate over the key-value pairs in the hash map
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // create a new empty string
    let mut s = String::new();

    // add characters to the string
    s.push('H');
    s.push_str("ello, ");
    s.push('w');
    s.push('o');
    s.push('r');
    s.push('l');
    s.push('d');

    // print the string
    println!("{}", s);
}

use clap::Command;
use clap::Arg;
use std::io::BufRead;
use std::io::BufReader;
use colored::Colorize;

enum Input {
    File(BufReader<std::fs::File>),
    Stdin(BufReader<std::io::Stdin>),
}

fn main() {

    let m = Command::new("fml")
        .author("Jonathan Aherne, jonathan@elevenchars.com")
        .version("0.1.0")
        .about("Highlights lines in a file or stdin that match a pattern")
        .arg(Arg::new("pattern").help("The search pattern").index(1))
        .arg(Arg::new("file").help("The input file (optional)").index(2).required(false))
        .get_matches();

    let pattern = m.get_one::<String>("pattern").expect("Pattern is required");
    println!("Value for pattern: {pattern}");
    let file = m.get_one::<String>("file");
    if file.is_some() {
        let input = file.unwrap();
        println!("Value for input file: {input}");
    } else {
        let _input = std::io::stdin();
        println!("Reading from stdin");
    }

    // Define the reader
    let reader: Input;

    // Open the file, assuming we have one
    if file.is_some() {
        let file = std::fs::File::open(file.unwrap()).expect("Could not open file");
        reader = Input::File(BufReader::new(file));
    }
    // Open stdin if we don't have a file
    else {
        reader = Input::Stdin(BufReader::new(std::io::stdin()));
    }

    match reader {
        Input::File(reader) => process(reader, &pattern),
        Input::Stdin(reader) => process(reader, &pattern),
    }
}

fn process<R: BufRead>(reader: R, pattern: &str) {
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if line.contains(pattern) {
            // Get the index of the pattern in the line
            let start = line.find(pattern).expect("Pattern not found");
            let end = start + pattern.len();

            // Print the line with the pattern highlighted
            print!("{}", &line[..start]);
            print!("{}", &line[start..end].bold().red());
            println!("{}", &line[end..]);
        } else {
            println!("{}", line);
        }
    }
}

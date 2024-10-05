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

    let mut cmd = Command::new("fml")
        .author("Jonathan Aherne, jonathan@elevenchars.com")
        .version("0.1.0")
        .about("Highlights lines in a file or stdin that match a pattern")
        .arg(Arg::new("pattern").help("The search pattern").index(1).required(true))
        .arg(Arg::new("file").help("The input file (optional)").index(2));
        // Todo: add option for additional color schemes
        // Todo: add option for case sensitivity
        // Todo: add option for regex
    let m = cmd.get_matches_mut();

    let pattern = m.get_one::<String>("pattern").expect("Pattern is required");
    println!("Value for pattern: {pattern}");
    let file = m.get_one::<String>("file");

    let reader: Input;

    // Open the file, assuming we have one
    if file.is_some() {
        println!("Opening file: {}", file.unwrap());
        let file = std::fs::File::open(file.unwrap()).expect("Could not open file");
        reader = Input::File(BufReader::new(file));
    } else { // Open stdin as we don't have a file
        reader = Input::Stdin(BufReader::new(std::io::stdin()));
    }

    match reader {
        Input::File(reader) => process(reader, &pattern),
        Input::Stdin(reader) => process(reader, &pattern),
    }
}

// Process the input reader agnostic, highlighting the pattern if found
fn process<R: BufRead>(reader: R, pattern: &str) {
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if line.contains(pattern) {
            // Get the index of the pattern in the line
            // Todo: can we combine find and contains?
            let start = line.find(pattern).expect("Pattern not found");
            let end = start + pattern.len();

            // Print the line with the pattern highlighted
            // Todo: check if this works with wide characters. Probably not
            print!("{}", &line[..start]);
            print!("{}", &line[start..end].bold().red());
            println!("{}", &line[end..]);
        } else {
            println!("{}", line);
        }
    }
}

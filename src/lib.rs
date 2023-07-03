use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::{Command, Arg, ArgAction};

/// `Config` struct is used to configure the parameters for file processing.
///
/// # Fields
///
/// * `files`: A vector of file names (Strings) that will be processed by the program.
/// * `count_lines`: A boolean value indicating whether to print line numbers or not.
/// * `nonblank_number`: A boolean value indicating whether to print line numbers for non-blank lines or not.
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    count_lines: bool,
    nonblank_number: bool,
}

/// Constructs a new Command for the `minicat` program.
///
/// # Description
///
/// The `minicat` is a simplified, Rust version of the `cat` Unix command for
/// concatenating and displaying file content. The function configures and returns a new
/// Command instance for the `minicat` command, specifying its version, about information, and arguments.
///
/// # Arguments
///
/// * `files`: appendable argument allowing users to specify the files to be read. Hyphen values are permitted.
/// * `number` ('-n'): this option will number all output lines.
/// * `nonblank` ('-b'): this option will number only nonblank lines.
///
/// Note: the `number` and `nonblank` options are mutually exclusive.
///
/// # Returns
///
/// * `Command` - A new Command instance configured for the `minicat` command.
///
/// # Example
///
/// ```
/// let matches = build_cli().get_matches();
/// ```
fn build_cli() -> Command {
    Command::new("minicat")
        .about("Rust version of the cat command")
        .version("0.1.0")
        .arg(Arg::new("files")
            .action(ArgAction::Append)
            .value_name("FILES")
            .default_value("")
            .help("Files to read")
            .allow_hyphen_values(true))
        .arg(Arg::new("number")
            .action(ArgAction::SetTrue)
            .short('n')
            .overrides_with("number")
            .help("Numbers the lines")
            .conflicts_with("nonblank"))
        .arg(Arg::new("nonblank")
            .action(ArgAction::SetTrue)
            .short('b')
            .overrides_with("nonblank")
            .help("Number only nonblank lines"))
}

/// The `get_args` function is used to parse command line arguments and return a Config struct.
///
/// # Arguments
///
/// None
///
/// # Returns
///
/// * `Result<Config, Box<dyn Error>>` - Returns a `Config` struct that contains the parsed command-line arguments.
/// In case of any errors, returns an error of type `Box<dyn Error>`.
///
/// # Errors
///
/// This function will return an error if there is a problem with parsing
/// the command line arguments, for example, missing required parameters or invalid flag values.
///
/// # Example
///
/// ```no_run
/// let config = get_args().unwrap();
/// println!("{:?}", config);
/// ```
pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = build_cli().get_matches();
    let files = matches
        .get_many("files")
        .expect("at least one file")
        .map(|x: &String| x.to_owned())
        .collect::<Vec<String>>();

    Ok(Config{
        files: files,
        count_lines: matches.get_flag("number"),
        nonblank_number: matches.get_flag("nonblank")
    })
}
/// This function accepts a `Config` object and processes each file included in the `Config` object's `files` vector.
/// It handles file opening, checking the lines, and printing.
///
/// # Arguments
///
/// * `config`: An instance of `Config` class which contains the configuration for the program. It includes line counting preference,
/// non-blank line counting preference, and the list of file names to be processed.
///
/// # Returns
///
/// * On success, an `Ok(())` is returned.
/// * On failure, an `Err` variant with a boxed `Error` instance is returned.
///
/// # Errors
///
/// The function will return an error if there is an issue when trying to open or read the lines of the files.
///
/// # Example
///
/// ```
/// let config = Config {
///     files: vec!["./src/main.rs", "./src/lib.rs"],
///     count_lines: true,
///     nonblank_number: false
/// };
///
/// match run(config) {
///     Ok(()) => println!("Files processed successfully."),
///     Err(e) => eprintln!("An error occurred: {}", e),
/// }
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for filename in config.files {
        match open_file(&filename) {
            Ok(file) => {
                // dbg!("Opened file {}", filename);
                let mut blank_count: usize = 0;
                for (number, line) in file.lines().enumerate() {
                    if let Ok(line) = line {
                        if config.count_lines {
                            println!("{}\t{}", number + 1, line);
                        } else if config.nonblank_number {
                            if line.is_empty() {
                                blank_count += 1;
                                println!("{}", line);
                            } else {
                                println!("{}\t{}", number + 1 - blank_count, line);
                            }
                        } else {
                            println!("{}", line);
                        }
                    }
                }
            },
            Err(e) => eprintln!("Failed to open {} due to {}", filename, e),
        }
    }

    Ok(())
}

/// Opens a file for reading or returns standard input stream if file string is empty.
///
/// ## Parameters
/// * `file` - A string slice reference which contains the path to the file. If it is an empty string, the function returns standard input stream.
///
/// ## Returns
/// A `std::io::Result` which is an alias for `Result<T, E>` where `E` is `std::io::Error`.
/// If successful, the function returns a `Box` containing a type implementing the `BufRead` trait.
///
/// # Errors
/// The function will return an error if `std::fs::File::open()` fails.
fn open_file(file: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match file {
        "" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?)))
    }
}

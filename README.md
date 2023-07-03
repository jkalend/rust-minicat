# Rust-Minicat

`rust-minicat` is a simplified version of the `cat` Unix command for concatenating and displaying file content written in Rust programming language. This command line tool comes with features such as line numbering and non-blank line numbering.

## Features

- Multiple file support: You can specify multiple files to read.
- Line numbering: It has an option to number all output lines.
- Non-blank line numbering: Only non-blank lines can be numbered if you want.

## Usage
For running the project, using `cargo run` is recommended, otherwise just run:
```shell
rust-minicat [FLAGS] [FILES]
```
 - FLAGS:
   - -n: Number all output lines.
   - -b: Number only non-blank output lines.
 
<i>Note: The number and nonblank options are mutually exclusive.</i>
 - FILES: Files to read

## Project Structure
The project consists of the single main file and library that contains all the logic and the config struct as well.

## Tests
No tests are provided as of now

## Contributing
We welcome contributions of all kinds to this project. Whether it's bug fixes, new features, or updating the documentation, your help is greatly appreciated!
Please feel free to create issues and submit pull requests on Github.

## License
This project is licensed under the MIT license

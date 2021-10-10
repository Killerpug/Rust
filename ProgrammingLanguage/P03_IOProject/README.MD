# 1. IO Project: Building a Command Line Program

Rust’s speed, safety, single binary output, and cross-platform support make it an ideal language for creating command line tools, so for our project, we’ll make our own version of the classic command line tool **grep** (globally search a regular expression and print). 

In the simplest use case, grep searches a specified file for a specified string. To do so, grep takes as its arguments a filename and a string. Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.  

We’ll read the value of an environment variable to allow the user to configure the behavior of our tool. We’ll also print error messages to the standard error console stream (stderr) instead of standard output (stdout), so, for example, the user can redirect successful output to a file while still seeing error messages onscreen.

If you want to see a fully featured grep, search for Andrew Gallant, he has created a fully featured, very fast version of grep, called [**ripgrep**](https://github.com/BurntSushi/ripgrep)

## 2. Features

- Finds a string within a file and prints lines that contains that string
- Uses command line arguments to input file and string to search
- Configurable case insensitive search via environment variable $Env:CASE_INSENSITIVE=1;
- Prints error messages to standard error console stream (stderr)

## 3. Concepts in practice

- Code module organization.
- Vectors, Strings.
- Error Handling.
- Traits and lifetimes.
- Tests.
- Environment variables

### 4. Use cases
Common search:  

- `cargo run {word} {poem.txt}`

redirect stderr data stream to output.txt:  

- `cargo run {word} {poem.txt} 2> output.txt`

Configuring case insensitive search:  

- `$Env:CASE_INSENSITIVE=1; cargo run to poem.txt`  

unset case insensitive:  

- `Remove-Item Env:CASE_INSENSITIVE`

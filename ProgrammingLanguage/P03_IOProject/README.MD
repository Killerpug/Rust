## IO Project: Building a Command Line Program
Rust’s speed, safety, single binary output, and cross-platform support make it an ideal language for creating command line tools, so for our project,we’ll make our own version of the classic command line tool **grep** (globally search a regular expression and print). In the simplest use case, grep searches a specified file for a specified string. To do so, grep takes as its arguments a filename and a string. Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.  
Along the way, we’ll show how to make our command line tool use features of the terminal that many command line tools use. We’ll read the value of an environment variable to allow the user to configure the behavior of our tool. We’ll also print error messages to the standard error console stream (stderr) instead of standard output (stdout), so, for example, the user can redirect successful output to a file while still seeing error messages onscreen.

### Concepts in practice ###
- Code module organization.
- Vectors, Strings.
- Error Handling.
- Traits and lifetimes.
- Tests.
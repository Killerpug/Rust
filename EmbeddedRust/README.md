# First time setup

1. Install [Rust ESP dependencies](https://esp-rs.github.io/book/installation)

2. source sdk to use espup on current terminal(each time you compile)
```
    . $HOME/export-esp.sh
```

3. Generate a project using std [template](https://esp-rs.github.io/book/writing-your-own-application/generate-project-from-template.html)

4. Compile and run
```
    cargo build
    cargo espflash 
```
4.1 for some reason, cargo run doesnt work yet, so we need to grant device permissions to open serial port
```
    sudo adduser $USER $(stat --format="%G" /dev/ttyACM0 )
```

5. Watching serial output
minicom -b 115200 -D /dev/ttyACM0 

## Create a new project from the template
1. Generate a project using std [template](https://esp-rs.github.io/book/writing-your-own-application/generate-project-from-template.html)
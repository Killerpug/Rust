//! Prints "Hello, world!" on the host console using semihosting
/* From the root (RE00_Template) you can compile and run project for qemu

To build this project:
 cargo build --example hello

To run this project:
qemu-system-arm \
  -cpu cortex-m3 \
  -machine lm3s6965evb \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel target/thumbv7m-none-eabi/debug/examples/hello

Remote debugging involves a client and a server. 
In a QEMU setup, the client will be a GDB (or LLDB) process and
the server will be the QEMU process that's also running the embedded program.
TO DEBUG with QEMU:
(server)
$ qemu-system-arm \
  -cpu cortex-m3 \
  -machine lm3s6965evb \
  -nographic \
  -semihosting-config enable=on,target=native \
  -gdb tcp::3333 \
  -S \
  -kernel target/thumbv7m-none-eabi/debug/examples/hello

  Then on another terminal (client):
  $ gdb-multiarch -q target/thumbv7m-none-eabi/debug/examples/hello 
  $ target remote :3333

  This will initiate a gdb connection on port 3333.
*/
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

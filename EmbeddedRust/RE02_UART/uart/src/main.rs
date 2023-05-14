use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::*;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");
    let peripherals = Peripherals::take().unwrap();
    let tx = peripherals.pins.gpio??;
    let rx = peripherals.pins.gpio??;

    println!("Starting UART loopback test");
    let config = config::Config::new().baudrate(Hertz(115_200));
    let uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        None,
        None,
        &config,
    )
    .unwrap();

    loop {
        uart.write(&[0xaa]).unwrap();

        let mut buf = [0_u8; 1];
        uart.read(&mut buf, BLOCK).unwrap();

        println!("Written 0xaa, read 0x{:02x}", buf[0]);
    }
}

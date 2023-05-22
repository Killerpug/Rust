use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::*;
use esp_idf_hal::delay::BLOCK;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio12).unwrap();
    let mut modem_pwrkey = PinDriver::output(peripherals.pins.gpio4).unwrap();
    let mut modem_flight = PinDriver::output(peripherals.pins.gpio25).unwrap();

    
    // turn on gps module
    modem_pwrkey.set_high().unwrap();
    FreeRtos::delay_ms(1000);
    modem_pwrkey.set_low().unwrap();
    modem_flight.set_high().unwrap();

    let tx = peripherals.pins.gpio1;
    let rx = peripherals.pins.gpio2;

    let config = config::Config::default().baudrate(Hertz(115_200));
    let mut uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<Gpio0>::None,
        Option::<Gpio1>::None,
        &config,
    )
    .unwrap();
    println!("setup initialized");
    led.set_high().unwrap();
    loop {
        FreeRtos::delay_ms(1000);
        led.set_low().unwrap();        
        FreeRtos::delay_ms(1000);
        led.set_high().unwrap();
        uart.write(&[0xaa]).unwrap();

        let mut buf = [0_u8; 1];
        uart.read(&mut buf, BLOCK).unwrap();

        println!("Written 0xaa, read 0x{:02x}", buf[0]);
    }
}

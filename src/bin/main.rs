#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::main;
use esp_hal::gpio::{
    Output,
    Level,
    OutputConfig
};
use esp_hal::time::Rate;
use esp_hal::rmt::{
    PulseCode, 
    Rmt, 
    RxChannelConfig,
    RxChannelCreator
};
use esp_hal::delay::Delay;
use esp_println::{print, println};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.1.0

    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    // Start up the receiver
    let mut receiver_cs = Output::new(peripherals.GPIO3, Level::High, OutputConfig::default()); // Start High to enable receiver
    receiver_cs.set_high();

    const WIDTH: usize = 80;

    // Configure frequency based on chip type
    let freq = Rate::from_mhz(80);
    let rmt = Rmt::new(peripherals.RMT, freq).expect("Could not bind remote");

    let rx_config = RxChannelConfig::default()
        .with_clk_divider(1)
        .with_idle_threshold(10000);
    let mut channel = rmt.channel2.configure_rx(peripherals.GPIO4, rx_config).expect("Channel could not be configured");
    let delay = Delay::new();
    let mut data: [PulseCode; 48] = [PulseCode::default(); 48];

    loop {
        for x in data.iter_mut() {
            x.reset()
        }

        let transaction = channel.receive(&mut data).expect("Transaction could not be received");

        match transaction.wait() {
            Ok((symbol_count, channel_res)) => {
                channel = channel_res;
                let mut total = 0usize;
                for entry in &data[..symbol_count] {
                    if entry.length1() == 0 {
                        break;
                    }
                    total += entry.length1() as usize;

                    if entry.length2() == 0 {
                        break;
                    }
                    total += entry.length2() as usize;
                }

                for entry in &data[..symbol_count] {
                    if entry.length1() == 0 {
                        break;
                    }

                    let count = WIDTH / (total / entry.length1() as usize);
                    let c = match entry.level1() {
                        Level::High => '-',
                        Level::Low => '_',
                    };
                    for _ in 0..count + 1 {
                        print!("{}", c);
                    }

                    if entry.length2() == 0 {
                        break;
                    }

                    let count = WIDTH / (total / entry.length2() as usize);
                    let c = match entry.level2() {
                        Level::High => '-',
                        Level::Low => '_',
                    };
                    for _ in 0..count + 1 {
                        print!("{}", c);
                    }
                }

                println!();
            }
            Err((_err, channel_res)) => {
                channel = channel_res;
            }
        }

        delay.delay_millis(1500);
    }
}

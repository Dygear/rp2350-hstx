#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts, gpio,
    peripherals::PIO0,
    pio::{InterruptHandler, Pio},
    pio_programs::ws2812::{PioWs2812, PioWs2812Program},
};
use embassy_time::{Duration, Ticker};
use gpio::{Level, Output};
use panic_probe as _;
use smart_leds::RGB8;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"NeoBlinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"Adafruit Feather RP2350 HSTX Blinks LED and NeoPixel"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Init");
    let p = embassy_rp::init(Default::default());
    // Onboard LED Setup
    let mut led = Output::new(p.PIN_7, Level::Low);

    // NeoPixel Setup
    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);
    const NUM_LEDS: usize = 1;
    let mut data = [RGB8::default(); NUM_LEDS];
    let program = PioWs2812Program::new(&mut common);
    // For the Adafruit RP2350 HSTX the NeoPixel PIN is 21.
    let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_21, &program);
    // Loop forever making RGB values and pushing them out to the WS2812.
    let mut ticker = Ticker::every(Duration::from_millis(25));

    loop {
        info!("Loop");
        led.toggle();
        for j in 0..(256 * 3) {
            for i in 0..NUM_LEDS {
                data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
            }
            ws2812.write(&data).await;
            ticker.next().await;
        }
    }
}

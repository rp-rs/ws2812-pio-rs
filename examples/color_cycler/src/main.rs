#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    pio::PIOExt,
    sio::Sio,
    watchdog::Watchdog,
};

use smart_leds_trait::SmartLedsWrite;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let (mut pio0, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let timer = bsp::hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let mut leds = ws2812_pio::Ws2812::new(
        pins.gpio15.into_function(),
        &mut pio0,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut t = 0.0;
    loop {
        // Cycle each component up and down, with each component/LED being 33% further in their period.
        let pattern: [[u8; 3]; 3] = core::array::from_fn(|i| {
            core::array::from_fn(|i2| {
                let x = (t + (i + i2) as f32 / 3.0) % 1.0 * 2.;
                (if x >= 1.0 { 2.0 - x } else { x } * 10.0) as u8
            })
        });
        leds.write(pattern.into_iter().cycle().take(50)).unwrap();
        delay.delay_ms(20);
        t += 0.02
    }
}

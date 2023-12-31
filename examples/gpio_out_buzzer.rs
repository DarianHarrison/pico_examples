//! # Pico PWM Melody Example
//!
//! Fades the LED on a Pico board using the PWM peripheral and plays a melody on a buzzer.
//!
//! This will fade in/out the LED attached to GP25 (on-board LED) and play a melody on the
//! buzzer attached to a GPIO pin (e.g., GP26).
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::entry;

// GPIO traits
use embedded_hal::PwmPin;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

const PWM_DIV: u8 = 40;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Buzzer
    let pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM7
    let mut buzzer = pwm_slices.pwm7;
    buzzer.set_ph_correct();

    // (CLK / DIV / FREQ * 2) == (12000000 / 40 / 261.63)
    fn calc_note(freq: f32) -> u16 {
        (rp_pico::XOSC_CRYSTAL_FREQ as f32 / PWM_DIV as f32 / freq) as u16
    }

  // Notes
    let c4 = calc_note(261.63); // do
    let _c4_sharp = calc_note(277.18); // do sostenido
    let d4 = calc_note(293.66); // re
    let _d4_sharp = calc_note(311.1); // re sostenido
    let e4 = calc_note(329.63); // mi
    let f4 = calc_note(349.23); // fa
    let _f4_sharp = calc_note(369.99);// fa sostenido
    let g4 = calc_note(392.00); // sol
    let _g4_sharp = calc_note(415.30);// sol sostenido
    let a4 = calc_note(440.00); // la
    let _a4_sharp = calc_note(466.16);// la sostenido
    let b4 = calc_note(493.88); // si
   
    let _space = 0;

    let doremi = [c4, d4, e4, f4, g4, a4, b4];
    let twinkle_twinkle = [
        b4,
        g4,
        e4,
        b4,
        g4,
        e4,
        a4,
        f4,
        d4,
        a4,
        f4,
        d4,
        g4,
        e4,
        c4,
        g4,
        e4,
        c4,
    ];

    buzzer.enable();
    buzzer.channel_b.output_to(pins.gpio15);
    buzzer.set_div_int(PWM_DIV);

    loop {
        // Play melody
        for &top in &doremi {
            buzzer.channel_b.set_duty(top / 2); // 50% Duty Cycle
            buzzer.set_top(top);
            delay.delay_ms(500);
        }
        buzzer.channel_b.set_duty(0);
        delay.delay_ms(1000);

        // Play melody 2
        for &top in &twinkle_twinkle {
            buzzer.channel_b.set_duty(top / 2); // 50% de ciclo de trabajo
            buzzer.set_top(top);
            delay.delay_ms(500);
        }
        buzzer.channel_b.set_duty(0);

        delay.delay_ms(500);
    }
}

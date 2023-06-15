# l10_detector


## reference

Edge
- [] 1. configure different clock frequencies ((5) different frequencies between min and max)
- [] 2. use pwm to blink a led (many times per second)
- [] 3. use adc to capture photoresistor (every 1 second)
- [] 4. send photoresist data to usb continous (every 1 second)

```sh
#cargo run --release --example frequency
#cargo run --release --example pwm
#cargo run --release --example adc
#cargo run --release --example usb_in
```

![Alt Text](./docs/ccd-datasheet.png)

3000-pixel CCD Linear Image Sensor (B/W) - ILX526A Datasheet (PDF) - Sony Corporation
https://pdf1.alldatasheet.com/datasheet-pdf/view/47503/SONY/ILX526A.html



![Alt Text](./docs/pico-datasheet.png)




Core
- [] 1. poll usb device in pin
- [] 2. deserialize & print (every 1 second)

```sh
#cargo run --example usb <vid> <pid>
```




compile and flash program automatically
```sh
cargo run --release --example pico_blinky
# cargo run --release --example pico_pwm_blink
# cargo run --release --example adc_usb
# cargo run --release --example pico_countdown_blinky
```

## sources


boards
https://github.com/rubberduck203/stm32f3-discovery/tags
https://github.com/riscv-rust/hifive1/tags
https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico

embedded-hal
https://crates.io/crates/embedded-hal

Raspberry Silicon RP2040 microcontroller
https://crates.io/crates/rp2040-hal
note: this is re-exported by the board crate, so it does not need to be in cargo.toml if you implement the board rp-pico board crate 

usb host system
https://github.com/rust-embedded-community/usbd-serial
https://github.com/a1ien/rusb
https://github.com/rust-embedded-community/usb-device


Future:
- [] dma
- [] dma to usb
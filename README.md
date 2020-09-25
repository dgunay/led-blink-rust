# Making an LED blink with Rust

Following this guide: https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0

This code makes an LED blink with Rust on the Arduino UNO.

## Uploading the code to the Arduino

```powershell
avrdude -p atmega328p -c arduino -P COM3 -D -U flash:w:rust-arduino-blink.elf
```

Note: the `-D` is of absolute importance, otherwise the bootloader will be
erased. I unfortunately learned this the hard way.

## Simulating the code with simavr

```bash
# Install simavr
sudo apt install simavr

# Compile the code
cargo build

# Load it into the simulator
simavr -m atmega328p target/avr-atmega328p/debug/rust-arduino-blink.elf
```

You can also debug the code.

```bash
# In one terminal
# -g flag lets it listen for a debug connection
simavr -g -m atmega328p target/avr-atmega328p/debug/rust-arduino-blink.elf
```

```bash
# In another terminal
avr-gdb ./rust-arduino-blink.elf
```

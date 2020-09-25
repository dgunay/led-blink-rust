# Making an LED blink with Rust

Following this guide: https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0

This code makes an LED blink with Rust on the Arduino Uno.

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

You can also debug the code with avr-gdb.

```bash
# In one terminal
# -g flag lets it listen for a debug connection
simavr -g -m atmega328p target/avr-atmega328p/debug/rust-arduino-blink.elf

# It will listen on port 1234
```

```bash
# In another terminal
avr-gdb ./rust-arduino-blink.elf

# In avr-gdb:
target remote :1234

# And you should be ready to debug
```

## Other notes

The code does compile under Windows with WinAVR. Debugging sort of works even if 
you use pc-windows-msvc but the GNU toolchain works better so I'd recommend 
that.

Debug messages only appear to work with simavr 1.6, not 1.5. I have not made
a serious attempt at compiling simavr for windows.
# Making an LED blink with Rust

Following this guide: https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0

This code makes an LED blink with Rust on the Arduino UNO.

```powershell
avrdude -p atmega328p -c arduino -P COM3 -D -U flash:w:rust-arduino-blink.elf
```

Note: the `-D` is of absolute importance, otherwise the bootloader will be
erased. I unfortunately learned this the hard way.
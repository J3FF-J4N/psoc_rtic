# Setup

## Toolchains

//This
Cortex-M0, M0+, and M1 (ARMv6-M architecture):
rustup target add thumbv6m-none-eabi

Cortex-M3 (ARMv7-M architecture):
rustup target add thumbv7m-none-eabi

//This
Cortex-M4 and M7 without hardware floating point (ARMv7E-M architecture):
rustup target add thumbv7em-none-eabi

Cortex-M4F and M7F with hardware floating point (ARMv7E-M architecture):
rustup target add thumbv7em-none-eabihf

Cortex-M23 (ARMv8-M architecture):
rustup target add thumbv8m.base-none-eabi

Cortex-M33 and M35P (ARMv8-M architecture):
rustup target add thumbv8m.main-none-eabi

Cortex-M33F and M35PF with hardware floating point (ARMv8-M architecture):
rustup target add thumbv8m.main-none-eabihf

## Tools

cargo install cargo-binutils

rustup component add llvm-tools

## Get a flashable file

run:

```
cargo build
```

followed by: 

```
objcopy -O ihex .\target\thumbv6m-none-eabi\debug\psoc6 output.hex
```

objcopy -O ihex .\target\thumbv6m-none-eabi\debug\psoc6 output.hex



# Notes

HAL and PAC is for different board but same CPU cores. 

Used board: CY8CKIT-062-BLE Pioneer board.

PAC: https://github.com/psoc-rs/psoc6-pac

Setup memory map for Rust: 
https://docs.rust-embedded.org/book/start/hardware.html
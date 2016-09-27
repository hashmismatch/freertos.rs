# freertos.rs

This library is a Rust interface for the [FreeRTOS](http://www.freertos.org/) API. Currently, it requires nightly Rust.

It is assumed that it will be used as a library in the firmware project for your embedded platform, with a C compiler providing the base system.
This library uses a C shim to communicate with FreeRTOS's APIs. Dynamic memory allocation must be provided.

[![Build Status](https://travis-ci.org/hashmismatch/freertos.rs.svg?branch=master)](https://travis-ci.org/hashmismatch/freertos.rs)

[Documentation](https://docs.rs/freertos_rs)

[Introduction article](...)

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
freertos_rs = "0.1"
```

Next, add this to your crate:

```rust
extern crate freertos_rs;

use freertos_rs::*;
```

## Unit tests

This project includes [unit tests](qemu_stm32_tests/src/) that are run on the [The GNU ARM Eclipse QEMU](http://gnuarmeclipse.github.io/qemu/) emulator. The basic firmware for STM32 is written in C, compiled with GCC and finally linked with the particular unit test's entry point. Since the QEMU emulator doesn't support FPU registers, slight changes were made to the FreeRTOS kernel. Lacking timer hardware emulation is simulated with a Systick-dependent IRQ trigger. Rust code invokes the cross compilation of this crate, compilation of the base firmware, final linkage and then runs the unit tests using the emulator.
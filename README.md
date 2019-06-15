# freertos.rs

A Rust wrapper for [FreeRTOS](http://www.freertos.org/). Beta Rust is required - soon to be stable.

The main entry point for your embedded executable should be provided by your platform's preffered compiler toolchain. For example, for STM microcontrollers, a project generated using STM32CubeMX and compiled using GCC could be used as a starting point. Additional shims between C and Rust provide access to drivers, hardware and FreeRTOS.

This library uses a C shim to communicate with FreeRTOS's API. Dynamic memory allocation is required at the moment.

[![Build Status](https://travis-ci.org/hashmismatch/freertos.rs.svg?branch=master)](https://travis-ci.org/hashmismatch/freertos.rs)

[![Documentation](https://docs.rs/freertos_rs/badge.svg)](https://docs.rs/freertos_rs)

[Introduction article](http://www.hashmismatch.net/freertos-meets-rust/)

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
freertos_rs = "0.2"
```

Next, add this to your project:

```rust
extern crate freertos_rs;

use freertos_rs::*;
```

## Unit tests

This project includes [unit tests](qemu_stm32_tests/src/) that are run on the [The GNU ARM Eclipse QEMU](http://gnuarmeclipse.github.io/qemu/) emulator. The basic firmware for STM32 is written in C, compiled with GCC and finally linked with the particular unit test's entry point. Since the QEMU emulator doesn't support FPU registers, slight changes were made to the FreeRTOS kernel. Lacking timer hardware emulation is simulated with a Systick-dependent IRQ trigger. Rust code invokes the cross compilation of this crate, compilation of the base firmware, final linkage and then runs the unit tests using the emulator.
# Starter template for a Windows driver written in Rust

## Driver

Contains the driver entry code.

## Kernel

Contains the library for interacting with the kernel.

## Compiling

`cd driver && cargo build --features winreg`

NOTE: winreg is disabled by default to allow development on a Linux machine. It is only used in the
build file and can be ignored during development.

### Remote Build Scripts

These scripts (located in remote_build_scripts) use SSH to copy and build on a Windows VM. Adjust to your liking.

### Resources

- https://not-matthias.github.io/posts/kernel-driver-with-rust-2022/
- https://github.com/StephanvanSchaik/windows-kernel-rs

# microbit-starter

A simple starter project for the micro:bit v2 using Rust.

## Prerequisites

You will need:

- A micro:bit v2
- Rust and Cargo installed
- cargo-embed installed

## Flashing

Connect your micro:bit v2 to your computer and run the following command:

```bash
cargo embed --target thumbv7em-none-eabihf
```

On your micro:bit v2, you should see the first LED light up.

## Credits

This is based on the setup project from the [Discovery book](https://docs.rust-embedded.org/discovery/microbit).

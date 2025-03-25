# `.cargo/config.toml` and `cargo run`
The `.cargo/config.toml` file configures `cargo run` to use [`probe-rs`](https://probe.rs) to flash to the boards ROM.
You will need a [Raspberry Pi Debug Probe](https://www.raspberrypi.com/products/debug-probe/) in order to use this, but it makes development MUCH easier, faster, and more fun!
You connect the Debug Probe's `D` (for debug, `D` for `defmt` :wink:) side to the board's [Debug Port](https://learn.adafruit.com/adafruit-feather-rp2350/pinouts#debug-port-3182499).
Once done connect the Adafruit board, and Debug Probe to your computer. You can flash at will with `cargo run` and see any debug messages in your computer's terminal thanks to [`defmt`](https://defmt.ferrous-systems.com).

# `memory.x`
If you've never seen a `memory.x` file before, and have no clue what it is; I don't blame you for being curious.
It's an odd file, filled with things that aren't Rust or C, or anything else that fits the norm.
This file actually tells the linker where to put sections of the binary.
It makes sure everything is in order so that when the microcontroller jumps to flash memory, the expected data is there ready for it.
It also tells the the linker how much RAM the target board or chip has.

# `build.rs`
This build script copies the `memory.x` file from the crate root into a directory where the linker can always find it at build time.
For many projects this is optional, as the linker always searches the project root directory -- wherever `Cargo.toml` is.
However, if you are using a workspace or have a more complicated build setup, this build script becomes required.
Additionally, by requesting that Cargo re-run the build script whenever `memory.x` is changed, updating `memory.x` ensures a rebuild of the application with the new memory settings.

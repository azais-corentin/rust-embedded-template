![build status](https://github.com/azais-corentin/rust-embedded-template/actions/workflows/build.yml/badge.svg)

# `rust-embedded-template`

> Quickly set up a [`probe-rs`] + [`defmt`] + [`flip-link`] embedded project

[`probe-rs`]: https://crates.io/crates/probe-rs
[`defmt`]: https://github.com/knurling-rs/defmt
[`flip-link`]: https://github.com/knurling-rs/flip-link

## Dependencies

#### 1. `flip-link`:

```console
$ cargo install flip-link
```

#### 2. `probe-rs`:

[Follow the installation instructions](https://probe.rs/docs/getting-started/installation/)

#### 3. [`cargo-generate`]:

```console
$ cargo install cargo-generate
```

[`cargo-generate`]: https://crates.io/crates/cargo-generate

> _Note:_ You can also just clone this repository instead of using `cargo-generate`, but this involves additional manual adjustments.

## Setup

#### 1. Initialize the project template

```console
$ cargo generate azais-corentin/rust-embedded-template
```

#### 2. Get a linker script

Some HAL crates require that you manually copy over a file called `memory.x` from the HAL to the root of your project. For nrf52840-hal, this is done automatically so no action is needed. For other HAL crates, you can get it from your local Cargo folder, the default location is under:

```
~/.cargo/registry/src/
```

Not all HALs provide a `memory.x` file, you may need to write it yourself. Check the documentation for the HAL you are using.

#### 3. Run!

You are now all set to `cargo-run` your first `defmt`-powered application!

Run `cargo run` to get started:

```console
$ # `rb` is an alias for `run --bin`
$ cargo run
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.02s
     Running `probe-rs run --chip STM32F072RBTx target\thumbv6m-none-eabi\debug\test`
      Erasing ✔ [00:00:00] [###############################] 7.00 KiB/7.00 KiB @ 24.84 KiB/s (eta 0s )
  Programming ✔ [00:00:00] [###############################] 7.00 KiB/7.00 KiB @ 19.62 KiB/s (eta 0s )
    Finished in 0.658s
INFO  Hello, world!
└─ test::__cortex_m_rt_main @ src\main.rs:14
```

If you're running out of memory (`flip-link` bails with an overflow error), you can decrease the size of the device memory buffer by setting the `DEFMT_RTT_BUFFER_SIZE` environment variable. The default value is 1024 bytes, and powers of two should be used for optimal performance:

```console
$ DEFMT_RTT_BUFFER_SIZE=64 cargo rb hello
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[Knurling]: https://knurling.ferrous-systems.com
[Ferrous Systems]: https://ferrous-systems.com/
[GitHub Sponsors]: https://github.com/sponsors/knurling-rs

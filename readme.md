# Bevy with Rust-GPU sample
This repository is sample for how to use [Bevy](https://bevyengine.org/) with [Rust-GPU](https://github.com/EmbarkStudios/rust-gpu).
You can use this as a template or just reference.

## How it works
`shader_builder` crate is tiny library to build shader crates in `shaders` directory and copy the output `.spv` to assets directory.
You can call it manually via `xtask` or automatically via `build.rs` in your `game` crate.
`cargo xtask --path <PATH_TO_CRATE>` will build your shader and Bevy will hot-reload it.

## Related things
* [`xtask`](https://github.com/matklad/cargo-xtask) is a workflow to add automation to Rust projects.
* [`bevy_pbr`](https://github.com/bevyengine/bevy/tree/main/crates/bevy_pbr) is the Bevy official PBR crate with WGSL shaders.
    It is really active and looks hard to maintain its equivalent Rust-GPU implements up-to-date.
* [`Bevy-Rust-GPU`](https://github.com/Bevy-Rust-GPU) is another project to use Bevy with Rust-GPU. Not sure it is active.
* [`bevy-pbr-rust`](https://github.com/Bevy-Rust-GPU/bevy-pbr-rust) is Rust reimplementation of `bevy_pbr`'s WGSL shaders.
    Part of the `Bevy-Rust-GPU` project.

## Limitation
* No support for ShaderDef.
* Since Bevy uses `wgpu` as renderer backend, shaders functions are limited to webgpu specs.
  - You may bypass this limitation switching the backend, though I don't know which renderer plugin is available and actively updated.

## Known problems
* Cannot set shader build target to webgpu: [issue](https://github.com/hyranno/bevy_with_rust_gpu_sample/issues/2)
* Unused shader inputs are getting eliminated: [issue](https://github.com/hyranno/bevy_with_rust_gpu_sample/issues/3)

## License
This repository is dual-licensed under MIT and Apache 2.0 at your option.


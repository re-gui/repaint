# repaint
 A basic painting library

An all-rust alternative to skia.

:warning: this is a work in progress, and is not yet ready for use.

## Provided implementations

In order of priority:
- [ ] on [rust-skia](https://github.com/rust-skia/rust-skia) (in progress)
- [ ] on [piet](https://github.com/linebender/piet)
- [ ] CPU rasterizer (based on custom and exported algorithms)
- [ ] with [wgpu](https://github.com/gfx-rs/wgpu)
- [ ] on [cairo]

where:
- "on ...": the implementation is a wrapper around the given crate, 
- "with ...": the implementation is custom but based on the given crate under the hood.
- no prefix: the implementation is custom.
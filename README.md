# repaint
 A basic 2d painting library.

:warning: this is a work in progress, and is not yet ready for use. It is being developed primarily for the [regui](https://github.com/re-gui/regui) crate.

## Provided implementations

In order of priority:

where:

| Implementation | Status | Notes |
| --- | --- | --- |
| [`repaint-with-skia-safe`](./implementations/skia-safe/) | :construction: :star: |  wrapper around [rust-skia](https://github.com/rust-skia/rust-skia) |
| [`repaint-on-html-canvas`](./implementations/html-canvas/) | :x: |  |
| PDF | :x: |  |
| [`repaint-with-piet`](./implementations/piet/) | :x: |  wrapper around [piet](https://github.com/linebender/piet) |
| [`repaint-with-rasterizer`](./implementations/rasterizer/) | :x: | CPU rasterizer for custom canvas (e.g. TFT devices) |
| [`repaint-with-cpu`](./implementations/cpu/) | :x: | bitmap rasterizer based on [repaint-with-rasterizer](./implementations/rasterizer/) |
| [`repaint-with-wgpu`](./implementations/wgpu/) | :x: |  custom implementation based on [wgpu](https://github.com/gfx-rs/wgpu) |
| [`repaint-with-sfml`](./implementations/sfml/) | :x: |  |
| [`repaint-with-cairo`](./implementations/cairo/) | :x: |  |
| [`repaint-with-glium`](./implementations/glium/) | :x: |  |
> :warning: TODO ...

Legend:
- :star: : the reference implementation
- :x: : not started
- :construction: : in progress
- :white_check_mark: : done

> :information_source: the [`repaint-with-skia-safe`](./implementations/skia-safe/) (currently) has the priority, `repaint` will be adapted to it and [`repaint-with-skia-safe`](./implementations/skia-safe/) can be considered as the reference implementation: refer to it when in doubt.

TODO serde

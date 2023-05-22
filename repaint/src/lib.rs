
/*!

Provides a unified API for 2d drawing on a canvas and a set of common algorithms.

## Introduction

In Rust (as well as in other languages) there are a lot of crates that provide a way to perform 2d graphics, all of them have their own API and their own way of doing things and it can be quite hard to find the right one for your needs. For example, if you want to make your own gui library, [Skia](https://skia.org/) could be probably the best choice, but it could be on overkill for a simple game and it would be a pain to integrate it in projects with completely different needs.

With *repaint*, you can base all your drawing logic on a unified API that is easy to use and than implement the necessary drawing logic for your specific needs, possibly changing backend without having to change the drawing logic.  
To achieve this, you can implement the [`Canvas`](`canvas::Canvas`) trait using [Skia](https://skia.org/), or implement your own canvas using a display on an embedded device, on a printer, or everywhere you want.  

Implementing your own low-level drawing logic is not always easy, so *repaint* also provides a set of common algorithms that can be used to implement the drawing logic for your canvas. As an example, if you are implementing your own canvas on an image and you don't know how to draw a line, you can use the [*Bresenham* algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm) to do it, but it could be tedious to implement it yourself, so *repaint* provides some common algorithms that can be used to implement the drawing logic for your canvas; in this case, you might want to use the [`base::rasterization::algorithms::line`] module and pick the algorithm that suits your needs.

## Architecture

In *repaint*, there are two main actors: the **painter** and the **canvas**.  
The *canvas* represents an object that can be drawn on, it does not expose any drawing methods, but rather provides a *painter* that can be used to draw on it.  
The *painter* is an object that can draw on a canvas. It exposes a set of methods that are used to draw on the canvas and allows for a high-level drawing experience, providing a rich and unified API.

*Canvas* and *painter* are represented by the [`Canvas`](`canvas::Canvas`) and [`Painter`](`painter::Painter`) traits respectively, the distinction is simple but quite important: the [`Canvas`](`canvas::Canvas`) abstracts the drawing surface that will have its own way of drawing, the [`Painter`](`painter::Painter`) is provided by the canvas in order to draw on it with a unified API while hiding the possibly complex drawing logic of the canvas.

As an example, suppose we are working with an Embedded setup that comprehends a display. The display will be wrapped, in the code, by some object and it has to perform a very complex set of operations in order to draw something on the screen. In order to separate all the complexity that using a [painter algorithm](https://en.wikipedia.org/wiki/Painter%27s_algorithm) might require from the actual drawing logic, we just implement the [`Canvas`](`canvas::Canvas`) trait for our display object, and then provide a [`Painter`](`painter::Painter`) that will properly use the display's methods to draw on it while implementing the painter's algorithm its own way.

An example of the final usage:
```rust
fn render(canvas: &mut dyn canvas::Canvas) {
    let mut painter = canvas.painter();
    // TODO ...
}
```

*/

//#![warn(missing_docs)]

pub use nalgebra;

pub mod base;
pub mod utils;
pub mod canvas;
pub mod painter;

pub use canvas::Canvas;
pub use painter::Painter;
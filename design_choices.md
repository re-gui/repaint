# Architecture

## `dyn Painter` vs `impl Painter`

This is a fundamental choice that affects the whole architecture.

| `dyn Painter`               | `impl Painter`                   |
|-----------------------------|----------------------------------|
| + Easy to use (no generics) | - Litte harder to use (generics) |
| + possibly smaller binary   | - possibly bigger binary         |
| - Dynamic dispatch          | + No dynamic dispatch            |

The main advantage of `dyn Painter` for the user is that it is easier to use, as it does not require generics.
As an example, resources such as an `Image` will be a fixed type that the user can easily store in a **state** struct.  
In other places, `dyn` will make the syntax a little bit more verbose as we would not be able to use things like `impl Iteratir` or `impl Fn` in some methods signatures and the user will be forced to add `&mut ...` when they call those methods and many optimizations would be impossible. Sure the compiler could optimize it under some conditions and fall in a static dispatch, but it is not guaranteed and it is not clear when it would happen.

Resources might be a real pain using dynamic dispatch: while the resource types will be the same for each paiter for the user point of view, it would be necessary to use dynamic allocation adn dispatch for those resources under the hood.  
As an example, an `Image` resource could look something like this:
```rust
struct ImageResource<'context> {
    raw: Box<dyn ImageRaw>,
    context: &'context RefCell<ContextInner>,
}
```
or something similar. This might be sub-optimal because it forces to have at least one dynamic allocation per resource, and this might not be necessary and it would be really problematic on embedded or MCU targets where we would like to avoid dynamic allocation as much as possible.

The `impl Painter` approach is a litte more verbose for the user, but it might allows to avoid dynamic allocation and dispatch for resources.  
For the `ImageExample`, the paiter could expose an `Image` type:
```rust
trait Painter {
    type Image<'context>: ImageResource;
}
```
Where the `Image` type is defined by the painter. If the user needs to cache the image, they coul use something like this:
```rust
struct State<P: Painter> {
    image: P::Image,
}
```

The `impl Painter` does not exclude the possiblity of using dynamic dispatch for the painter itself. As an example, we could define something like this:
```rust
trait DynPainterImpl {// <- PRIVATE
    // ...
}
impl<P: Painter> P for DynPainterImpl {
    // ...
}

pub struct DynPainter<'painter> { // <- PUBLIC
    raw: &'painter mut dyn DynPainterImpl,
}

impl<'painter> Painter for DynPainter<'painter> {
    // ...
}
```

This allows to wrap any `impl Painter` into a common type that can be used to abstract over the different painters.

## Resources

Resources are managed in a `context`. A context is responsible, among other things, for resource management.
Every resource lives in a context, and cannot escape it.

A context is identified subdivided into `ContextInner` and `Context`:
 - The `ContextInner` is the actual context, it is not `Send` nor `Sync` and is not exposed to the user. (or shouln't ???)
 - The `Context` is a wraps `ContextInner` and exposes a safe interface to the user.

The `Context` holds the `ContextInner` using a `RefCell`. Resources holds a reference to the refcell in order to ensure that the context is alive as long as the resources are alive.

When an operation is performed on a resource, the resource borrows the `ContextInner` mutably.
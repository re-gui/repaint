# Architecture

## Resources

Resources are managed in a `context`. A context is responsible, among other things, for resource management.
Every resource lives in a context, and cannot escape it.

A context is identified subdivided into `ContextInner` and `Context`:
 - The `ContextInner` is the actual context, it is not `Send` nor `Sync` and is not exposed to the user. (or shouln't ???)
 - The `Context` is a wraps `ContextInner` and exposes a safe interface to the user.

The `Context` holds the `ContextInner` using a `RefCell`. Resources holds a reference to the refcell in order to ensure that the context is alive as long as the resources are alive.

When an operation is performed on a resource, the resource borrows the `ContextInner` mutably.
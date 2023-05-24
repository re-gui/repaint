/*!
Provides the methods for the Painter struct grouped into
distinct traits.
*/

mod blend;
mod antialias;
mod transform;
mod drawing;
mod shapes;

pub use blend::*;
pub use antialias::*;
pub use transform::*;
pub use drawing::*;
pub use shapes::*;
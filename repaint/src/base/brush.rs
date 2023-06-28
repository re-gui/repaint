use crate::painter::WithPathResource;


pub trait Brush<Painter: WithPathResource + ?Sized> {
    //fn sweep(
    //    &self,
    //    painter: &mut Painter,
    //    path: &Painter::Path,
    //);
}

// TODO some basic brushes like solid color, lines, dots, squares, tassellation, ...


/*pub trait AbstractPainterResource {}

pub trait PainterResource<'painter> {}

pub trait IntoPainterResource<'painter> {
    type Resurce: PainterResource<'painter>;
    fn into_resource(self, painter: &'painter dyn Painter) -> Self::Resurce; // TODO or impl?
}

impl<'painter, R: PainterResource<'painter>> IntoPainterResource<'painter> for R {
    type Resurce = R;
    fn into_resource(self, _painter: &'painter dyn Painter) -> Self::Resurce {
        self
    }
}*/


/*pub struct PainterResource<'context, HandleType: ?Sized> { // TODO any necessary?
    pub handle: Rc<HandleType>,
    //_phantom: std::marker::PhantomData<&'context_lifecycle str>, // TODO correct?
    //s: &'painter str,
    context: &'context RefCell<>
}

impl<'context_lifecycle, HandleType: ?Sized> PainterResource<'context_lifecycle, HandleType> {
    pub fn new(
        handle: Rc<HandleType>,
        lifecycle: &'context_lifecycle (),
    ) -> Self {
        Self {
            handle,
            _phantom: std::marker::PhantomData,
        }
    }
}*/

//pub struct S<T: 'static>(T);

//type A<'painter> = S<PainterResource<'painter>>;

//struct B(i32);


//pub fn ciao<'s>(s: &'s str) -> Rc<A<'s>> {
//    let b = B(0);
//    let a = S(PainterResource::<'s> {
//        handle: Rc::new(b),
//        //_phantom: std::marker::PhantomData,
//        s,
//    });
//    Rc::new(a)
//}
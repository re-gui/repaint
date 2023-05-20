
struct Foo {
}

impl Foo {
    fn get_ref(&mut self) -> FooRef {
        FooRef {
            foo_ref: self,
        }
    }
}

struct FooRef<'a> {
    foo_ref: &'a mut Foo,
}

impl Default for Foo {
    fn default() -> Self {
        Self {}
    }
}

fn main() {
    let mut f = Foo::default();

    let mut fr = f.get_ref();
    let mut fr2 = f.get_ref();
}


use facet_macros::Facet;
use facet_reflect::{Peek, PeekStruct};

#[derive(Facet)]
struct DataHolder<'a> {
    data: &'a mut i32,
}


#[test]
fn contravariance_example_for_mutable_references() {
    fn force_long<'long, 'short>(x: &'long mut i32) -> &'short mut i32
    where
        'long: 'short,
    {
        x
    }
    let mut value = 42;
    let long: &mut i32 = &mut value;

    let short1 = force_long(long);
    let short2 = force_long(long);
    assert_eq!(*short1, 42);
    assert_eq!(*short2, 42);
}

#[test]
fn contravariance_example_for_mutable_references_via_peek() {
    let mut value = 42;
    let data_holder = DataHolder { data: &mut value };
    let peek = Peek::new(&data_holder).into_struct().unwrap();
    fn force_long<'long, 'short>(peek: PeekStruct<'long, 'long>) -> &'short mut i32
    where
        'long: 'short,
    {
        *peek.field_by_name("data").unwrap().get::<&mut i32>().unwrap()
    }

    let short1 = force_long(peek);
    let short2 = force_long(peek);
    assert_eq!(*short1, 42);
    assert_eq!(*short2, 42);
}

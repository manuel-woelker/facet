use std::marker::PhantomData;
use facet_macros::Facet;
use facet_reflect::Peek;

type ProduceStaticStr<'a> = fn(&'a str) -> &'static str;

#[derive(Facet)]
struct FunctionHolder<'a> {
    function: ProduceStaticStr<'a>,
    phantom_data: PhantomData<fn(&'a ())>,
}

impl <'a> FunctionHolder<'a> {
    fn new(function: ProduceStaticStr<'a>) -> Self {
        Self {
            function,
            phantom_data: PhantomData,
        }
    }
}

#[test]
fn test_cast_away_the_static_life_time() {
    let function: ProduceStaticStr<'static> = |s| s;
    let function_holder = FunctionHolder::new(function);
    let peek = Peek::new(&function_holder);
    fn cast_away_to_static<'a>(input: &'a str, function: &ProduceStaticStr<'a>) -> &'static str {
        function(input)
    }

    let function_ref = peek.into_struct().unwrap().field_by_name("function").unwrap().get::<ProduceStaticStr>().unwrap();
    let temporary_string = "Definitely not static".to_string();
    let temporary_ref = temporary_string.as_str();
    
    let static_str: &'static str = cast_away_to_static(temporary_ref, function_ref);

    assert_eq!(static_str, "Definitely not static");
}

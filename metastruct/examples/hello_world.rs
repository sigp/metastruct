use metastruct::{metastruct, selectors::AllFields, NumFields};
use std::marker::PhantomData;

#[metastruct(
    mappings(
        map_numeric_fields_of_obj(exclude(y)),
        map_mut_numeric_fields_of_obj(exclude(y), mutable),
    ),
    num_fields(all(), numeric(selector = "NumericFields", exclude(y)))
)]
pub struct Obj {
    pub x: u64,
    pub y: String,
    pub z: u8,
    #[metastruct(exclude)]
    pub _phantom: PhantomData<()>,
}

fn sum(obj: &Obj) -> usize {
    let mut total = 0usize;
    map_numeric_fields_of_obj!(obj, |_, x| total += *x as usize);
    total
}

fn increment_all(obj: &mut Obj) {
    map_mut_numeric_fields_of_obj!(obj, |_, x| {
        *x += 1;
    });
}

fn main() {
    let mut obj = Obj {
        x: 10,
        y: "Hello world".to_string(),
        z: 5,
        _phantom: PhantomData,
    };

    println!("initial sum: {}", sum(&obj));
    increment_all(&mut obj);
    println!("after increment all: {}", sum(&obj));

    println!(
        "num fields? {}/{} are numeric",
        <Obj as NumFields<NumericFields>>::NUM_FIELDS,
        <Obj as NumFields<AllFields>>::NUM_FIELDS
    );
}

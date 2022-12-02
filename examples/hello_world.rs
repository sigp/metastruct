use metastruct::metastruct;

#[metastruct(mappings(
    map_numeric_fields_of_obj(exclude(y)),
    map_mut_numeric_fields_of_obj(exclude(y), mutable)
))]
pub struct Obj {
    pub x: u64,
    pub y: String,
    pub z: u8,
}

// FIXME(sproul): generate this
pub trait NumFields<Selector> {
    const NUM_FIELDS: usize;
}

pub struct AllFields;
pub struct AllNumericFields;

impl NumFields<AllFields> for Obj {
    const NUM_FIELDS: usize = 3;
}

impl NumFields<AllNumericFields> for Obj {
    const NUM_FIELDS: usize = 2;
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
    };

    println!("initial sum: {}", sum(&obj));
    increment_all(&mut obj);
    println!("after increment all: {}", sum(&obj));

    println!(
        "num fields? {}/{} are numeric",
        <Obj as NumFields<AllNumericFields>>::NUM_FIELDS,
        <Obj as NumFields<AllFields>>::NUM_FIELDS
    );
}

use metastruct_macro::metastruct;

#[metastruct(mappings(map_foo_fields()))]
struct Foo {
    x: u64,
    y: u16,
    z: u32,
}

fn sum<'a>(total: &'a mut u64, foo: &'a Foo) {
    map_foo_fields!(&'a _, foo, |_, field| *total += *field as u64);
}

#[test]
fn reference_with_lifetime() {
    let foo = Foo { x: 1, y: 2, z: 3 };
    let mut total = 0;
    sum(&mut total, &foo);
    assert_eq!(total, 6);
}

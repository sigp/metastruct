use metastruct_macro::metastruct;

#[metastruct(bimappings(
    bimap_foo(other_type = "Foo", self_mutable, other_by_value),
    bimap_foo_into_foo(other_type = "IntoFoo", self_mutable, other_by_value)
))]
#[derive(Debug, Clone, PartialEq)]
pub struct Foo {
    a: u64,
    b: u64,
    #[metastruct(exclude_from(copy))]
    c: String,
}

pub struct MyString(String);

impl From<MyString> for String {
    fn from(m: MyString) -> Self {
        m.0
    }
}

/// Type that has fields that can be converted to Foo's fields using `Into`
pub struct IntoFoo {
    a: u32,
    b: u32,
    c: MyString,
}

#[test]
fn bimap_self() {
    let mut x_foo = Foo {
        a: 0,
        b: 1,
        c: "X".to_string(),
    };
    let y_foo = Foo {
        a: 1000,
        b: 2000,
        c: "Y".to_string(),
    };

    bimap_foo!(&mut x_foo, y_foo.clone(), |_, x, y| {
        *x = y;
    });

    assert_eq!(x_foo, y_foo);
}

#[test]
fn bimap_into() {
    let mut x_foo = Foo {
        a: 0,
        b: 1,
        c: "X".to_string(),
    };
    let y_foo = IntoFoo {
        a: 1000,
        b: 2000,
        c: MyString("Y".to_string()),
    };

    fn set_from<T: From<U>, U>(x: &mut T, y: U) {
        *x = y.into();
    }

    bimap_foo_into_foo!(&mut x_foo, y_foo, |_, x, y| {
        set_from(x, y);
    });

    assert_eq!(x_foo.a, 1000);
    assert_eq!(x_foo.b, 2000);
    assert_eq!(x_foo.c, "Y");
}

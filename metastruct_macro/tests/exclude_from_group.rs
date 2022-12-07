use metastruct_macro::metastruct;

#[metastruct(mappings(
    neither_group(),
    only_group1(groups(group1)),
    only_group2(groups(group2)),
    both_groups(groups(group1, group2))
))]
pub struct Foo {
    both: u64,
    #[metastruct(exclude_from(group1))]
    group2: u64,
    #[metastruct(exclude_from(group2))]
    group1: u64,
    #[metastruct(exclude_from(group1, group2))]
    neither: u64,
}

fn sum_and_count_group1(foo: &Foo) -> (usize, u64) {
    let mut count = 0_usize;
    let mut total = 0_u64;
    only_group1!(foo, |_, x| {
        count += 1;
        total += *x
    });
    (count, total)
}

fn sum_and_count_group2(foo: &Foo) -> (usize, u64) {
    let mut count = 0_usize;
    let mut total = 0_u64;
    only_group2!(foo, |_, x| {
        count += 1;
        total += *x
    });
    (count, total)
}

fn sum_and_count_both(foo: &Foo) -> (usize, u64) {
    let mut count = 0_usize;
    let mut total = 0_u64;
    both_groups!(foo, |_, x| {
        count += 1;
        total += *x
    });
    (count, total)
}

fn sum_and_count_neither(foo: &Foo) -> (usize, u64) {
    let mut count = 0_usize;
    let mut total = 0_u64;
    neither_group!(foo, |_, x| {
        count += 1;
        total += *x
    });
    (count, total)
}

fn run_test(foo: Foo) {
    let (count1, sum1) = sum_and_count_group1(&foo);
    assert_eq!(count1, 2);
    assert_eq!(sum1, foo.both + foo.group1);

    let (count2, sum2) = sum_and_count_group2(&foo);
    assert_eq!(count2, 2);
    assert_eq!(sum2, foo.both + foo.group2);

    let (count_neither, sum_neither) = sum_and_count_neither(&foo);
    assert_eq!(count_neither, 4);
    assert_eq!(
        sum_neither,
        foo.both + foo.group1 + foo.group2 + foo.neither
    );

    let (count_both, sum_both) = sum_and_count_both(&foo);
    assert_eq!(count_both, 1);
    assert_eq!(sum_both, foo.both);
}

#[test]
fn exclude_from_groups() {
    run_test(Foo {
        both: 1,
        group2: 2,
        group1: 3,
        neither: 4,
    });
    run_test(Foo {
        both: 99,
        group2: 1000,
        group1: 2,
        neither: 777777777,
    });
}

fn main() {
    println!("Hello, world!");
}

// {:?} で表示できるようにしないと、テストで eq できない
#[derive(Debug)]
struct A {
    value: i32,
}

impl PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[test]
fn test() {
    let a = A {value: 3};
    let aa = A {value: 3};

    assert_eq!(a, aa);
}

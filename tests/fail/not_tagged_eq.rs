use tagged_id::{Id, Identify};

struct TestThis();
struct TestThat();

impl Identify for TestThis {
    type InnerId = i32;
}

impl Identify for TestThat {
    type InnerId = i32;
}

fn main() {
    let id1: Id<TestThis> = 42.into();
    let id2: Id<TestThat> = 42.into();

    assert_eq!(id1, id2);
}

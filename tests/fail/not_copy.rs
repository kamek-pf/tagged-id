use tagged_id::{Id, Identify};

fn main() {
    #[derive(Identify)]
    #[tagged_id(String)]
    struct TestStruct();

    let id1: Id<TestStruct> = "oops".into();
    let id2 = id1;

    assert_eq!(id1, id2);
}

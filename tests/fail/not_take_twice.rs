use smartstring::alias::CompactString;
use tagged_id::{Id, Identify};

fn main() {
    struct TestStruct();

    impl Identify for TestStruct {
        type InnerId = CompactString;
    }

    let id1: Id<TestStruct> = "oops".into();
    let id2 = id1.take();

    assert_eq!(id1.take(), id2);
}

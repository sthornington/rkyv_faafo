mod cache;
mod mmapbox;

use std::path::PathBuf;

use rkyv::ser::allocator::Arena;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(
    // This will generate a PartialEq impl between our unarchived
    // and archived types
    compare(PartialEq),
    // Derives can be passed through to the generated type:
    derive(Debug),
)]
struct Test {
    int: u8,
    string: String,
    option: Option<Vec<i32>>,
}

fn main() -> Result<(), anyhow::Error> {
    let value = Test {
        int: 42,
        string: "hello world".to_string(),
        option: Some(vec![1, 2, 3, 4]),
    };

    let mut arena = Arena::new();

    let path: PathBuf = "test.bin".into();
    let cached = cache::get_cached::<_, _, anyhow::Error>(&path, || value, arena.acquire())?;

    assert_eq!(&cached, &value);
    Ok(())
}

mod cache;
mod mmapbox;
mod wuuid;

use std::collections::BTreeMap;
use std::path::PathBuf;

use rkyv::ser::allocator::Arena;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
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
    map: BTreeMap<u32, String>,
    id: wuuid,
}

fn main() -> Result<(), anyhow::Error> {
    let items = [(1, "one".into()), (2, "two".into())];
    let value = Test {
        int: 42,
        string: "hello world".to_string(),
        option: Some(vec![1, 2, 3, 4]),
        map: BTreeMap::from(items),
    };
    let expected = value.clone();
    let mut arena = Arena::new();

    let path: PathBuf = "test.bin".into();
    let cached = cache::get_cached::<_, _, anyhow::Error>(&path, || value, arena.acquire())?;

    println!("{:?}", &expected);
    println!("{:?}", &cached);
    //assert_eq!(cached.unchecked(), &expected);
    for (k, v) in expected.map.iter() {
        println!("{} -> {}", k, v);
    }
    for (k, v) in cached.map.iter() {
        println!("{} -> {}", k, v);
    }
    Ok(())
}

mod cache;
mod mmapbox;
mod wuuid;
use std::collections::BTreeMap;
use std::hint::black_box;
use std::path::PathBuf;
use std::time::Instant;
use uuid;

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
    #[rkyv(with = wuuid::UuidDef)]
    id: uuid::Uuid,
}

fn main() -> Result<(), anyhow::Error> {
    let items = [(1, "one".into()), (2, "two".into())];
    let value = Test {
        int: 42,
        string: "hello world".to_string(),
        option: Some(vec![1, 2, 3, 4]),
        map: BTreeMap::from(items),
        id: uuid::Uuid::new_v4(),
    };
    let expected = value.clone();
    let mut arena = Arena::new();

    let path: PathBuf = "test.bin".into();
    const N: usize = 1000000;
    let mut cached: Option<_> = None;
    let start = Instant::now();
    for _ in 0..N {
        cached = Some(cache::get_cached::<_, _, anyhow::Error>(
            &path,
            || value.clone(),
            arena.acquire(),
        )?);
        black_box(cached.as_ref().unwrap());
    }
    let elapsed = start.elapsed();
    println!("{:?}ns per iteration", elapsed.as_nanos() / (N as u128));

    println!("{:?}", &expected);
    println!("{:?}", cached.as_ref().unwrap());
    //assert_eq!(cached.unchecked(), &expected);
    for (k, v) in expected.map.iter() {
        println!("{} -> {}", k, v);
    }
    for (k, v) in cached.as_ref().unwrap().map.iter() {
        println!("{} -> {}", k, v);
    }
    Ok(())
}

use std::fmt::Display;
use std::io;
use std::path::Path;

use crate::mmapbox::MmapBox;
use bytecheck::CheckBytes;
use rkyv::Archive;
use rkyv::Portable;
use rkyv::Serialize;
use rkyv::api::high::HighValidator;
use rkyv::rancor;
use rkyv::rancor::Strategy;
use rkyv::ser::Serializer;
use rkyv::ser::allocator::ArenaHandle;
use rkyv::ser::sharing::Share;
use rkyv::util::AlignedVec;

pub fn get_cached<'a, T, F, E>(
    path: &Path,
    generate: F,
    arena_handle: ArenaHandle<'a>,
) -> Result<MmapBox<T, E>, E>
where
    T: Archive + Serialize<Strategy<Serializer<AlignedVec, ArenaHandle<'a>, Share>, rancor::Error>>,
    T::Archived: Portable + for<'b> CheckBytes<HighValidator<'b, rancor::Error>>,
    F: FnOnce() -> T,
    E: From<io::Error> + Display + From<rancor::Error>,
{
    if !path.exists() {
        let value = generate();
        let bytes: AlignedVec =
            rkyv::api::high::to_bytes_with_alloc::<_, rancor::Error>(&value, arena_handle)?;
        std::fs::write(path, bytes.as_ref())?;
    }

    MmapBox::<_, E>::new(path)
}

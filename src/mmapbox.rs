use bytecheck::CheckBytes;
use memmap2::{Mmap, MmapOptions};
use rkyv::Archive;
use rkyv::Portable;
use rkyv::api::high::HighValidator;
use rkyv::rancor;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::Path;

// Zero-copy memory-mapped container
pub struct MmapBox<T: Archive, E> {
    mmap: Mmap,
    phantom: PhantomData<T>,
    phantom_error: PhantomData<E>,
    // We will store a pointer or use Deref to get &Archived<T>
}

impl<T, E> MmapBox<T, E>
where
    T: Archive,
    T::Archived: Portable + for<'a> CheckBytes<HighValidator<'a, rancor::Error>>,
    E: From<io::Error> + Display + From<rancor::Error>,
{
    pub fn new(path: &Path) -> Result<Self, E> {
        let file = File::open(path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        //et _ = rkyv::access::<T::Archived, rancor::Error>(&mmap)?;
        Ok(MmapBox {
            mmap,
            phantom: PhantomData,
            phantom_error: PhantomData,
        })
    }

    pub fn unchecked(&self) -> &T::Archived {
        unsafe { rkyv::access_unchecked::<T::Archived>(&self.mmap) }
    }
}

// Implement Deref for convenient access to archived data
impl<T, E> Deref for MmapBox<T, E>
where
    T: Archive,
    T::Archived: Portable + for<'a> CheckBytes<HighValidator<'a, rancor::Error>>,
    E: From<io::Error> + Display + From<rancor::Error>,
{
    type Target = T::Archived;
    fn deref(&self) -> &Self::Target {
        self.unchecked()
    }
}

impl<T, E> fmt::Debug for MmapBox<T, E>
where
    T: Archive,
    T::Archived: Portable + for<'a> CheckBytes<HighValidator<'a, rancor::Error>> + std::fmt::Debug,
    E: From<io::Error> + Display + From<rancor::Error>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.unchecked())
    }
}

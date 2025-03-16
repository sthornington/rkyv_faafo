use rkyv::{Archive, Serialize, Deserialize, Fallible};
use rkyv::with::{ArchiveWith, SerializeWith, DeserializeWith};
use uuid::Uuid;
use core::ptr;

/// A wrapper type to customize how `Uuid` is archived (zero-copy).
struct WrapperUuid;

impl ArchiveWith<Uuid> for WrapperUuid {
    // Store UUID as 16-byte array in archived form
    type Archived = [u8; 16];
    type Resolver = ();

    unsafe fn resolve_with(field: &Uuid, _pos: usize, _resolver: Self::Resolver, out: *mut Self::Archived) {
        // Copy the 16 bytes of the Uuid into the archive output.
        ptr::copy_nonoverlapping(field.as_bytes().as_ptr(), out.cast::<u8>(), 16);
    }
}

impl<S: Fallible + ?Sized> SerializeWith<Uuid, S> for WrapperUuid {
    fn serialize_with(field: &Uuid, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        // Write the raw bytes of the UUID into the serializer’s buffer
        serializer.write(field.as_bytes()).map(|_| ())
    }
}

impl<D: Fallible + ?Sized> DeserializeWith<[u8; 16], Uuid, D> for WrapperUuid {
    fn deserialize_with(field: &[u8; 16], _deserializer: &mut D) -> Result<Uuid, D::Error> {
        // Reconstruct Uuid from the archived 16-byte array
        Ok(Uuid::from_bytes(*field))
    }
}

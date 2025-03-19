use rkyv::{Archive, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
#[rkyv(remote = uuid::Uuid)]
#[rkyv(archived = ArchivedUuid)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct UuidDef(#[rkyv(getter = uuid::Uuid::as_bytes)] [u8; 16]);

impl From<UuidDef> for Uuid {
    fn from(value: UuidDef) -> Self {
        Uuid::from_bytes(value.0)
    }
}

impl PartialEq<Uuid> for ArchivedUuid {
    fn eq(&self, other: &Uuid) -> bool {
        self.0 == *other.as_bytes()
    }
}

use parserprinter::db::db_ops::ShortCommitRecord;
use serde::{ser::SerializeStruct, Serialize, Serializer};
pub struct ShortCommitRecordWrapper(pub ShortCommitRecord);

impl Serialize for ShortCommitRecordWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ShortCommitRecordWrapper", 3)?;
        s.serialize_field("hash", &self.0.hash)?;
        s.serialize_field("branch", &self.0.branch)?;
        s.serialize_field("message", &self.0.message)?;
        s.end()
    }
}

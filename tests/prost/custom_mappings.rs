pub mod uuid_as_bytes {
    // Implementation from https://stackoverflow.com/questions/65268226/rust-deserialization-converting-vector-of-bytes-to-hashset-of-uuid
    use uuid::Uuid;

    pub fn to_scalar(uuid: &Uuid) -> Vec<u8> {
        let mut res = Vec::with_capacity(16);
        res.extend_from_slice(uuid.as_bytes());
        res
    }

    pub fn from_scalar(proto: Vec<u8>) -> anyhow::Result<Uuid> {
        Ok(Uuid::from_slice(&proto)?)
    }
}

pub mod uuid_as_string {
    use std::str::FromStr;
    use uuid::Uuid;

    pub fn to_scalar(uuid: &Uuid) -> String {
        uuid.to_string()
    }

    pub fn from_scalar(proto: String) -> anyhow::Result<Uuid> {
        Ok(Uuid::from_str(&proto)?)
    }
}

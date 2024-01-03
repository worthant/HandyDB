#[derive(Clone, Debug)]
pub struct KvPair {
    pub key: String,
    pub value: String,
}

impl KvPair {
    pub fn new(key: String, value: String) -> Self {
        KvPair { key, value }
    }
}

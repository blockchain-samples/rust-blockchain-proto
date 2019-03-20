pub struct Block {
    pub data: String,
    pub hash:  String,
    pub last_hash: String,
}

impl Block {
    pub fn new(data: String, hash:  String, last_hash: String) -> Block {
        Block { data, hash, last_hash }
    }

    pub fn hello(data: &str) -> String {
        format!("{}*", data)
    }
}
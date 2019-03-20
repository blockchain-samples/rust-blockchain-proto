use crate::bl::Block;

pub struct Chain {
    pub blocs: Vec<Block>,
}

impl Chain {
    pub fn new() -> Chain {
        Chain { blocs: vec![
            Block::new(String::from("gen-data"), String::from("gen-hash"), String::from("gen-last_hash"))
        ]}
    }

    pub fn add_block_second(blocs: &mut Vec<Block>, data: String) {
        let last_hash = blocs[blocs.len() - 1].hash.clone();

        let mut com_string = String::new();

        com_string.push_str(data.as_ref());

        com_string.push_str(" <- ".as_ref());

        com_string.push_str(last_hash.as_ref());

        //let comString = String::from("{}{}", data,last_hash);

        let hash = lightning_hash(&com_string);

        let block = Block::new(data, hash, last_hash);

        blocs.push(block);
    }
}

fn lightning_hash(data: &str) -> String {
    format!("{}*", data)
}
mod bl;
mod chain;


fn main() {

    let hash_first = String::from("first");
    let hash_second = String::from("second");
    let myblock = bl::Block::new(String::from("some data"), hash_first, hash_second);

    println!("========================================================");

    println!("| {} |-| {} |-| {} |", myblock.data, myblock.hash, myblock.last_hash);
    println!("str: {}", bl::Block::hello("test"));

    let mut new_chain_second = chain::Chain::new();

    let vector_of_data = vec!["one", "two", "three"];

    println!("========================================================");

    for i in 0..=2 {
        chain::Chain::add_block_second(&mut new_chain_second.blocs, vector_of_data[i].to_string());
        println!("{} block added!", i + 1 );
    }

    println!("========================================================");

    let mut i = 1;

    for item_block in &new_chain_second.blocs {
        println!("{}.) | {} |-| {} |-| {} |", i, item_block.data, item_block.hash, item_block.last_hash);
        i += 1;
    }

    println!("========================================================");
}

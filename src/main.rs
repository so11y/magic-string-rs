use std::collections::HashMap;

use magic_string_rs::Chunk;

fn main() {
    let mut chunk = Chunk::new(0, 2, "xiha");

    {
        chunk.split(1);

        unsafe {
            println!("{:#?}", *chunk.next.unwrap().previous.unwrap());
        }
    }
}

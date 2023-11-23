use std::collections::HashMap;

use crate::{split_chuck, Chunk};

pub struct MagicString {
    pub byte_start: HashMap<usize, *mut Chunk>,
    pub byte_end: HashMap<usize, *mut Chunk>,
    pub root_chunk: Chunk,
    pub prev_chunk: *mut Chunk,
    intro: String,
    outro: String,
}
impl MagicString {
    pub fn new(content: &str) -> Self {
        let mut chunk = Chunk::new(0, content.len(), content);
        let prev_chunk = &mut chunk as *mut Chunk;
        return Self {
            byte_start: HashMap::new(),
            byte_end: HashMap::new(),
            intro: String::new(),
            outro: String::new(),
            root_chunk: chunk,
            prev_chunk,
        };
    }
    pub fn overwrite(&mut self, start: usize, end: usize, content: &str) -> Result<(), String> {
        split_chuck(self, start)?;
        split_chuck(self, end)?;
        let first = self.byte_start.get(&start);
        if first.is_some() {
            unsafe {
                let chunk = &mut **(first.unwrap());
                chunk.edit(content);
            }
        }
        Ok(())
    }

    pub fn to_string(&mut self) -> String {
        let mut str = self.intro.clone();
        let mut chunk = Some(&self.root_chunk);
        while chunk.is_some() {
            let cur = chunk.unwrap();
            str += cur.to_string().as_str();

            if cur.next.is_some() {
                chunk = Some(cur.next.as_ref().unwrap().as_ref());
            } else {
                break;
            }
        }
        return str + self.outro.as_str();
    }
}

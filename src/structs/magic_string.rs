use std::collections::HashMap;

use crate::{split_chunk, Chunk};

pub struct MagicString {
    pub byte_start: HashMap<usize, *mut Chunk>,
    pub byte_end: HashMap<usize, *mut Chunk>,
    pub root_chunk: Box<Chunk>,
    pub prev_chunk: *mut Chunk,
    intro: String,
    outro: String,
}
impl MagicString {
    pub fn new(content: &str) -> Self {
        let mut chunk = Box::new(Chunk::new(0, content.len(), content));
        let prev_chunk = &mut *chunk as *mut Chunk;
        let mut byte_start = HashMap::new();
        let mut byte_end = HashMap::new();
        byte_start.insert(0, prev_chunk);
        byte_end.insert(content.len(), prev_chunk);
        return Self {
            byte_start,
            byte_end,
            intro: String::new(),
            outro: String::new(),
            root_chunk: chunk,
            prev_chunk,
        };
    }
    pub fn overwrite(&mut self, start: usize, end: usize, content: &str) -> Result<(), String> {
        split_chunk(self, start)?;
        split_chunk(self, end)?;
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
        let mut chunk = Some(&*self.root_chunk);
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

    pub fn prepend(&mut self, content: &str) {
        self.intro += content;
    }

    pub fn append(&mut self, content: &str) {
        self.outro += content;
    }

    pub fn append_left(&mut self, index: usize, content: &str) -> Result<(), String> {
        if self.byte_start.get(&index).is_none() {
            split_chunk(self, index)?;
        }
        if self.byte_end.get(&index).is_none() {
            split_chunk(self, index)?;
        }
        let chunk = self
            .byte_start
            .get(&index)
            .unwrap_or(self.byte_end.get(&index).unwrap());

        unsafe {
            let chunk = &mut **chunk;
            chunk.intro += content;
        }
        Ok(())
    }

    pub fn remove(&mut self, start: usize, end: usize) -> Result<(), String> {
        if self.byte_start.get(&start).is_none() {
            split_chunk(self, start)?;
        }
        if self.byte_end.get(&end).is_none() {
            split_chunk(self, end)?;
        }
        vec![
            self.byte_start.get(&start).unwrap(),
            self.byte_start.get(&end).unwrap(),
        ]
        .into_iter()
        .for_each(|chunk| unsafe {
            let chunk = &mut **chunk;
            chunk.remove()
        });
        Ok(())
    }
}

use std::collections::HashMap;

use crate::Chunk;

pub struct MagicString {
    pub byte_start: HashMap<usize, *mut Chunk>,
    pub byte_end: HashMap<usize, *mut Chunk>,
    pub root_chunk: Box<Chunk>,
    pub prev_chunk: *mut Chunk,
    original: String,
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
        Self {
            byte_start,
            byte_end,
            original: content.to_string(),
            intro: String::new(),
            outro: String::new(),
            root_chunk: chunk,
            prev_chunk,
        }
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

    pub fn prepend(&mut self, content: &str) {
        self.intro += content;
    }

    pub fn append(&mut self, content: &str) {
        self.outro += content;
    }

    pub fn append_left(&mut self, index: usize, content: &str) -> Result<(), String> {
        split_chunk(self, index)?;

        let chunk = self.byte_end.get(&index);

        if chunk.is_some() {
            unsafe {
                let chunk = &mut **chunk.unwrap();
                chunk.append_left(content);
            }
        } else {
            self.intro += content;
        }

        Ok(())
    }

    pub fn remove(&mut self, start: usize, end: usize) -> Result<(), String> {
        split_chunk(self, start)?;
        split_chunk(self, end)?;
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

    pub fn has_changed(&self) -> bool {
        self.original != self.to_string()
    }
}

pub fn split_chunk(ms: &mut MagicString, index: usize) -> Result<(), String> {
    if ms.byte_start.contains_key(&index) || ms.byte_end.contains_key(&index) {
        return Ok(());
    }
    let mut perv_chunk = Some(unsafe { &mut *ms.prev_chunk });

    while perv_chunk.is_some() {
        let cur = perv_chunk.unwrap();
        if cur.contain(index) {
            return chunk_link(ms, cur, index);
        }
        let next = cur.next.as_mut();
        if next.is_some() {
            perv_chunk = Some(next.unwrap());
        } else {
            return Ok(());
        }
    }
    Ok(())
}

pub fn chunk_link(m: &mut MagicString, chunk: &mut Chunk, index: usize) -> Result<(), String> {
    if chunk.edited && chunk.content.len() > 0 {
        return Err(String::from(
            "Cannot split a chunk that has already been edited",
        ));
    }
    let new_chunk = chunk.split(index).unwrap();

    m.byte_end
        .insert(new_chunk.end, &mut **new_chunk as *mut Chunk);
    m.byte_start.insert(index, chunk as *mut Chunk);
    m.byte_end.insert(index, chunk as *mut Chunk);
    Ok(())
}

impl ToString for MagicString {
    fn to_string(&self) -> String {
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
}

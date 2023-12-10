use std::collections::HashMap;

use crate::Chunk;

pub struct MagicString {
    pub by_start: HashMap<usize, *mut Chunk>,
    pub by_end: HashMap<usize, *mut Chunk>,
    pub first_chunk: Box<Chunk>,
    pub last_searched_chunk: *mut Chunk,
    original: String,
    intro: String,
    outro: String,
}

impl MagicString {
    pub fn new(content: &str) -> Self {
        let mut chunk = Box::new(Chunk::new(0, content.len(), content));
        let last_searched_chunk = &mut *chunk as *mut Chunk;
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
        split(self, start)?;
        split(self, end)?;
        let first = self.by_start.get(&start);
        if first.is_some() {
            unsafe {
                let chunk = &mut **(first.unwrap());
                chunk.edit(content);
            }
        }
        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut str = self.intro.clone();
        let mut chunk = Some(&*self.first_chunk);
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

    pub fn has_changed(&mut self) -> bool {
        return self.original != self.to_string();
    }

    pub fn append_left(&mut self, index: usize, content: &str) -> Result<(), String> {
        split(self, index)?;

        let chunk = self.by_end.get(&index);

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
        split(self, start)?;
        split(self, end)?;

        let mut chunk = Some(self.by_start.get(&start).unwrap());

        while chunk.is_some() {
            let cur = unsafe { &mut **chunk.take().unwrap() };
            cur.remove();
            cur.edit("");

            if end > cur.end {
                chunk = self.by_start.get(&cur.end)
            }
        }

        Ok(())
    }

    pub fn has_changed(&self) -> bool {
        self.original != self.to_string()
    }
}

pub fn split(ms: &mut MagicString, index: usize) -> Result<(), String> {
    if ms.by_start.contains_key(&index) || ms.by_end.contains_key(&index) {
        return Ok(());
    }
    let mut chunk = Some(unsafe { &mut *ms.last_searched_chunk });

    let search_forward = index > chunk.as_ref().unwrap().end;

    while chunk.is_some() {
        let cur = chunk.take().unwrap();
        if cur.contain(index) {
            return split_chunk(ms, cur, index);
        }

        let next_chunk = if search_forward {
            ms.by_start.get(&cur.end)
        } else {
            ms.by_end.get(&cur.start)
        };
        if next_chunk.is_some() {
            chunk = Some(unsafe { &mut **next_chunk.unwrap() });
        }
    }
    Ok(())
}

pub fn split_chunk(m: &mut MagicString, chunk: &mut Chunk, index: usize) -> Result<(), String> {
    if chunk.edited && chunk.content.len() > 0 {
        return Err(String::from(
            "Cannot split a chunk that has already been edited",
        ));
    }
    let new_chunk = chunk.split(index).unwrap();

    let new_chunk_point = &mut **new_chunk as *mut Chunk;

    m.by_end.insert(new_chunk.end, new_chunk_point);
    m.by_start.insert(index, new_chunk_point);
    let chunk_point = chunk as *mut Chunk;
    m.by_end.insert(index, chunk_point);

    m.last_searched_chunk = chunk_point;
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

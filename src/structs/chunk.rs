use crate::MagicString;

#[derive(Debug, Clone)]
pub struct Chunk {
    start: usize,
    end: usize,
    original: String,
    content: String,
    pub(crate) intro: String,
    pub(crate) outro: String,
    pub(crate) next: Option<Box<Chunk>>,
    pub(crate) previous: Option<*mut Chunk>,
    pub(crate) edited: bool,
}

impl Chunk {
    pub fn new(start: usize, end: usize, content: &str) -> Chunk {
        return Chunk {
            start,
            end,
            original: content.to_string(),
            content: content.to_string(),
            intro: String::new(),
            outro: String::new(),
            next: None,
            previous: None,
            edited: false,
        };
    }

    pub fn append_left(&mut self, content: &str) {
        self.outro += content;
    }

    pub fn append_right(&mut self, content: &str) {
        self.intro = self.intro.clone() + content;
    }

    pub fn contain(&self, index: usize) -> bool {
        return index >= self.start && index <= self.end;
    }

    pub fn split(&mut self, index: usize) -> Option<&mut Box<Chunk>> {
        let slice_index = index - self.start;
        let original_before = self.original[..slice_index].to_string();
        let original_after = self.original[slice_index..].to_string();
        self.original = original_before.clone();
        let mut new_chunk = Chunk::new(index, self.end, &original_after);
        self.end = index;
        self.content = original_before;
        new_chunk.next = self.next.take();
        new_chunk.previous = Some(self as *mut Chunk);
        self.next = Some(Box::new(new_chunk));
        return self.next.as_mut();
    }

    pub fn to_string(&self) -> String {
        return self.intro.clone() + self.content.as_str() + self.outro.as_str();
    }

    pub fn edit(&mut self, content: &str) {
        self.content = content.to_string();
        self.edited = true;
    }

    pub fn remove(&mut self) {
        self.content.clear();
        self.intro.clear();
        self.outro.clear();
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
    return Ok(());
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

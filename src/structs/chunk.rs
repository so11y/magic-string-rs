#[derive(Debug)]
pub struct Chunk {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) original: String,
    pub(crate) content: String,
    pub(crate) intro: String,
    pub(crate) outro: String,
    pub(crate) next: Option<Box<Chunk>>,
    pub(crate) previous: Option<*mut Chunk>,
    pub(crate) edited: bool,
}

impl Chunk {
    pub fn new(start: usize, end: usize, content: &str) -> Self {
        Self {
            start,
            end,
            original: content.to_string(),
            content: content.to_string(),
            intro: String::new(),
            outro: String::new(),
            next: None,
            previous: None,
            edited: false,
        }
    }

    pub fn append_left(&mut self, content: &str) {
        self.outro += content;
    }

    pub fn append_right(&mut self, content: &str) {
        self.intro = self.intro.clone() + content;
    }

    pub fn contain(&self, index: usize) -> bool {
        index >= self.start && index <= self.end
    }

    pub fn split(&mut self, index: usize) -> Option<&mut Box<Chunk>> {
        let slice_index = index - self.start;
        let chars: Vec<char> = self.original.chars().collect();

        let original_before: String = chars[..slice_index].iter().collect();
        let original_after: String = chars[slice_index..].iter().collect();
        let mut new_chunk = Chunk::new(index, self.end, &original_after);
        self.original = original_before.clone();
        self.end = index;
        self.content = original_before;
        new_chunk.next = self.next.take();
        new_chunk.previous = Some(self as *mut Chunk);
        self.next = Some(Box::new(new_chunk));
        return self.next.as_mut();
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

impl ToString for Chunk {
    fn to_string(&self) -> String {
        self.intro.clone() + self.content.as_str() + self.outro.as_str()
    }
}
impl Clone for Chunk {
    fn clone(&self) -> Self {
        let mut chunk = Chunk::new(self.start, self.end, self.original.as_str());
        chunk.intro = self.intro.clone();
        chunk.outro = self.outro.clone();
        chunk.content = self.content.clone();
        chunk.edited = self.edited;

        chunk
    }
}

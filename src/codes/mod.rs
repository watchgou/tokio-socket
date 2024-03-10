pub mod encode;
pub mod decode;


pub enum ByteCode {
    Max,
}

impl ByteCode {
    pub fn size(&self) -> usize {
        match self {
            Self::Max => 8 * 1024 * 1024,
        }
    }
}

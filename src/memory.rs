use std::ops::{Index, IndexMut};

const LC3_MEMORY_SIZE: usize = 1 << 16;

pub struct Memory([u16; LC3_MEMORY_SIZE]);

impl Memory {
    pub fn new() -> Self {
        Self([0; LC3_MEMORY_SIZE])
    }

    pub fn write_at(&mut self, values: &[u16], offset: usize) {
        let slice = &mut self.0[offset..offset + values.len()];
        slice.copy_from_slice(values);
    }
}

impl Index<u16> for Memory {
    type Output = u16;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

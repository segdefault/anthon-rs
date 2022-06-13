use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BitString {
    bits: usize,
    capacity: usize,
}

impl BitString {
    pub fn new(bits: usize, capacity: usize) -> Self {
        BitString { bits, capacity }
    }

    pub fn bits(&self) -> usize {
        self.bits
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn get(&self, index: usize) -> bool {
        self.bits & (1 << index) > 0
    }

    pub fn set(&mut self, index: usize) {
        self.bits |= 1 << index;
    }

    pub fn unset(&mut self, index: usize) {
        self.bits = !self.bits;
        self.bits |= 1 << index;
        self.bits = !self.bits;
    }
}

impl From<&BitString> for Vec<bool> {
    fn from(bits: &BitString) -> Self {
        (0..bits.capacity())
            .into_iter()
            .map(|i| bits.get(i))
            .collect()
    }
}

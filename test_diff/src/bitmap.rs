use std::ops::{AddAssign, BitXor};

pub struct Bitmap {
    bytes: Vec<u8>,
    length: usize,
}

impl Bitmap {
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            length: 0,
        }
    }

    pub fn push(&mut self, value: bool) {
        let value = if value {
            1u8
        } else {
            0u8
        };

        if self.length % 8 == 0 {
            self.bytes.push(value)
        } else {
            if let Some(byte) = self.bytes.last_mut() {
                let bit = 1u8 << self.length % 8;
                byte.add_assign(bit * value);
            }
        }

        self.length += 1
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
        self.length += 8;
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.length {
            return None;
        }

        let vec_index = index / 8;
        let bit_index = index % 8;
        let cmp_byte = 1u8 << bit_index;
        let byte = self.bytes.get(vec_index)?;
        let bit = cmp_byte & byte;

        Some(cmp_byte == bit)
    }

    pub fn xor(&self, other: &Bitmap) -> Bitmap {
        let mut result = Bitmap::new();

        for (a, b) in self.bytes.iter().zip(&other.bytes) {
            result.push_byte(a.bitxor(b))
        }

        result
    }

    pub fn where_bit_is(&self, value: bool) -> Vec<usize> {
        let mut indices = vec![];

        for index in 0..self.length {
            if let Some(bit) = self.get(index) {
                if bit == value {
                    indices.push(index);
                }
            }
        }

        indices
    }
}

#[cfg(test)]
mod tests {
    use crate::bitmap::Bitmap;

    #[test]
    fn push_byte_true() {
        let mut bitmap = Bitmap::new();
        bitmap.push_byte(1);

        assert_eq!(bitmap.get(0), Some(true))
    }

    #[test]
    fn push_byte_false() {
        let mut bitmap = Bitmap::new();
        bitmap.push_byte(1);

        assert_eq!(bitmap.get(1), Some(false))
    }

    #[test]
    fn push_byte_out_of_bounds() {
        let mut bitmap = Bitmap::new();
        bitmap.push_byte(1);

        assert_eq!(bitmap.get(8), None)
    }

    #[test]
    fn push_bit_true() {
        let mut bitmap = Bitmap::new();
        bitmap.push(true);

        assert_eq!(bitmap.get(0), Some(true))
    }

    #[test]
    fn push_bit_false() {
        let mut bitmap = Bitmap::new();
        bitmap.push(false);

        assert_eq!(bitmap.get(0), Some(false))
    }

    #[test]
    fn push_bit_out_of_bounds() {
        let mut bitmap = Bitmap::new();
        bitmap.push(false);

        assert_eq!(bitmap.get(8), None)
    }

    #[test]
    fn push_multiple_bits() {
        let mut bitmap = Bitmap::new();
        bitmap.push(false);
        bitmap.push(true);
        bitmap.push(true);
        bitmap.push(false);
        bitmap.push(false);

        let values = vec![
            bitmap.get(4),
            bitmap.get(3),
            bitmap.get(2),
            bitmap.get(1),
            bitmap.get(0),
        ];

        let expected = vec![
            Some(false),
            Some(false),
            Some(true),
            Some(true),
            Some(false),
        ];

        assert_eq!(values, expected)
    }

    #[test]
    fn xor() {
        let mut a = Bitmap::new();
        a.push_byte(0b10101011u8);

        let mut b = Bitmap::new();
        b.push_byte(0b10110010);

        assert_eq!(a.xor(&b).bytes[0], 0b00011001u8)
    }

    #[test]
    fn xor_where() {
        let mut a = Bitmap::new();
        a.push_byte(0b10101011u8);

        let mut b = Bitmap::new();
        b.push_byte(0b10110010);

        let indices = vec![
            0,
            3,
            4,
        ];

        assert_eq!(a.xor(&b).where_bit_is(true), indices)
    }
}
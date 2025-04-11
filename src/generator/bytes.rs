use crate::settings::{Endian, Orientation};

pub struct Bytes {
    string: String,
    orientation: Orientation,
    endian: Endian,
}

impl Bytes {
    pub fn new(string: String, orientation: &Orientation, endian: &Endian) -> Self {
        return Self {
            string,
            orientation: orientation.clone(),
            endian: endian.clone(),
        };
    }

    pub fn dimensions(&self) -> (u32, u32) {
        let byte_length = self.string.as_bytes().len() as u32;
        let w = match self.orientation {
            Orientation::Horizontal => byte_length,
            Orientation::Vertical => 8,
        };
        let h = match self.orientation {
            Orientation::Horizontal => 8,
            Orientation::Vertical => byte_length,
        };
        return (w, h);
    }

    fn bit_values(&self) -> Vec<Vec<bool>> {
        let mut chars: Vec<Vec<bool>> = Vec::new();
        let raw_bytes = self.string.as_bytes();

        (0..raw_bytes.len()).for_each(|char_i| {
            let mut bits: Vec<bool> = Vec::new();
            let char: u8 = raw_bytes[char_i];

            (0..8).for_each(|bit| {
                let mask: u8 = 1 << bit;
                bits.push((char & mask) > 0);
            });

            chars.push(bits);
        });

        return chars;
    }
}

#[cfg(test)]
mod bytes_tests {
    use super::Bytes;
    use crate::settings::{Endian, Orientation};

    #[test]
    fn test_bits() {
        let bytes = Bytes::new(String::from("z"), &Orientation::Vertical, &Endian::Most);
        assert_eq!(
            vec![vec![false, true, false, true, true, true, true, false]],
            bytes.bit_values()
        );
    }

    #[test]
    fn test_dimensions() {
        let bytes = Bytes::new(String::from("abc"), &Orientation::Horizontal, &Endian::Most);
        assert_eq!((3, 8), bytes.dimensions());
        let bytes = Bytes::new(String::from("abc"), &Orientation::Vertical, &Endian::Most);
        assert_eq!((8, 3), bytes.dimensions());
    }
}

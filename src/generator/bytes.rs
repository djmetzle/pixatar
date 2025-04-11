use crate::settings::{Endian, Orientation};

pub struct Bytes {
    string: String,
    values: Vec<Vec<bool>>,
    orientation: Orientation,
    endian: Endian,
    current: u32,
}

impl Iterator for Bytes {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        let byte_length = self.string.as_bytes().len() as u32;
        match self.orientation {
            Orientation::Horizontal => {
                if self.current == 8 {
                    return None;
                }
                let bit = self.current;
                let index = match self.endian {
                    Endian::Most => bit as usize,
                    Endian::Least => 7 - bit as usize,
                };
                let row = self
                    .values
                    .iter()
                    .map(|char_bits| char_bits[index])
                    .collect();
                self.current += 1;
                return Some(row);
            }
            Orientation::Vertical => {
                if self.current == byte_length {
                    return None;
                }
                let char = self.current;
                let row = self.values[char as usize].clone();
                self.current += 1;
                return match self.endian {
                    Endian::Most => Some(row),
                    Endian::Least => Some(row.into_iter().rev().collect()),
                };
            }
        }
    }
}

impl Bytes {
    pub fn new(string: String, orientation: &Orientation, endian: &Endian) -> Self {
        let values = Self::bit_values(&string);
        return Self {
            string,
            values,
            orientation: orientation.clone(),
            endian: endian.clone(),
            current: 0,
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

    fn bit_values(string: &String) -> Vec<Vec<bool>> {
        let mut chars: Vec<Vec<bool>> = Vec::new();
        let raw_bytes = string.as_bytes();

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
        assert_eq!(
            vec![vec![false, true, false, true, true, true, true, false]],
            Bytes::bit_values(&String::from("z"))
        );
    }

    #[test]
    fn test_dimensions() {
        let bytes = Bytes::new(String::from("abc"), &Orientation::Horizontal, &Endian::Most);
        assert_eq!((3, 8), bytes.dimensions());
        let bytes = Bytes::new(String::from("abc"), &Orientation::Vertical, &Endian::Most);
        assert_eq!((8, 3), bytes.dimensions());
    }

    #[test]
    fn test_horizontal_iteration() {
        let bytes = Bytes::new(String::from("abc"), &Orientation::Horizontal, &Endian::Most);
        let mut row_count: usize = 0;
        for row in bytes {
            assert_eq!(row.len(), 3);
            row_count += 1;
        }
        assert_eq!(row_count, 8);
    }

    #[test]
    fn test_horizontal_iteration_values() {
        let mut bytes = Bytes::new(String::from("abc"), &Orientation::Horizontal, &Endian::Most);
        assert_eq!(bytes.next().unwrap(), vec![true, false, true]);
        assert_eq!(bytes.next().unwrap(), vec![false, true, true]);
        for _ in 0..6 {
            bytes.next();
        }
        assert_eq!(bytes.next(), None);
    }

    #[test]
    fn test_vertical_iteration() {
        let bytes = Bytes::new(String::from("abc"), &Orientation::Vertical, &Endian::Most);
        let mut row_count: usize = 0;
        for row in bytes {
            assert_eq!(row.len(), 8);
            row_count += 1;
        }
        assert_eq!(row_count, 3);
    }

    #[test]
    fn test_vertical_iteration_values() {
        let mut bytes = Bytes::new(String::from("az"), &Orientation::Vertical, &Endian::Most);
        let a = vec![true, false, false, false, false, true, true, false];
        let z = vec![false, true, false, true, true, true, true, false];
        assert_eq!(bytes.next().unwrap(), a);
        assert_eq!(bytes.next().unwrap(), z);
        assert_eq!(bytes.next(), None);
    }

    #[test]
    fn test_horizontal_endian_values() {
        let mut bytes = Bytes::new(
            String::from("abc"),
            &Orientation::Horizontal,
            &Endian::Least,
        );
        assert_eq!(bytes.next().unwrap(), vec![false, false, false]);
        assert_eq!(bytes.next().unwrap(), vec![true, true, true]);
        for _ in 0..6 {
            bytes.next();
        }
        assert_eq!(bytes.next(), None);
    }

    #[test]
    fn test_vertical_endian_values() {
        let mut bytes = Bytes::new(String::from("az"), &Orientation::Vertical, &Endian::Least);
        let a = vec![false, true, true, false, false, false, false, true];
        let z = vec![false, true, true, true, true, false, true, false];
        assert_eq!(bytes.next().unwrap(), a);
        assert_eq!(bytes.next().unwrap(), z);
        assert_eq!(bytes.next(), None);
    }
}

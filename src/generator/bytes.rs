pub struct Bytes {
    string: String,
}

impl Bytes {
    pub fn new(string: String) -> Self {
        return Self { string };
    }

    pub fn len(&self) -> u32 {
        let raw_bytes = self.string.as_bytes();
        return raw_bytes.len() as u32;
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

    #[test]
    fn test_bits() {
        let bytes = Bytes::new(String::from("z"));
        assert_eq!(
            vec![vec![false, true, false, true, true, true, true, false]],
            bytes.bit_values()
        );
    }
}

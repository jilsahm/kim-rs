pub struct KimString {
    bytes: Vec<u8>,
}

impl From<&str> for KimString {

    fn from(s: &str) -> Self {
        let mut bytes = Vec::with_capacity(s.len());
        for c in s.chars() {
            let mut buffer = [0u8; 4];
            c.encode_utf8(&mut buffer);

            match c.len_utf8() {
                1 => {
                    // ASCII mapped 1 to 1
                    bytes.push(buffer[0] & 0b_0111_1111);
                }
                2 => {
                    // UTF-8 ß = 11000011 10011111
                    // KIM ß   = 10000001 01011111
                    let b1 = 0b_1000_0000 | ((buffer[0] & 0b_00011111) >> 1);
                    bytes.push(b1);
                    let b2 = 0b_0000_0000 | ((buffer[0] << 6) & 0b_01000000) | buffer[1] & 0b_0011_1111;
                    bytes.push(b2);
                }
                3 => {
                    // UTF-8 ☃ = 11100010 10011000 10000011
                    // KIM ☃   = 10000000 11001100 00000011   

                    // UTF-8 ♲ = 11100010 10011001 10110010
                    // KIM ♲   = 10000000 11001100 01110010

                    let b1 = 0b_1000_0000 | ((buffer[0] >> 2) & 0b_0000_0011);
                    bytes.push(b1);
                    let mut b2 = 0b_1000_0000;
                    b2 = b2 | ((buffer[0] << 5) & 0b_0110_0000);
                    b2 = b2 | ((buffer[1] >> 1) & 0b_0001_1111);
                    bytes.push(b2);
                    let mut b3 = 0b_0000_0000;
                    b3 = b3 | ((buffer[1] << 6) & 0b_0100_0000);
                    b3 = b3 | (buffer[2] & 0b_0011_1111);
                    bytes.push(b3);
                }
                4 => {
                    // UTF-8 𓂀 = 11110000 10010011 10000010 10000000
                    // KIM 𓂀   = 10000100 11100001 00000000

                    // UTF-8 𓃠 = 11110000 10010011 10000011 10100000
                    // KIM 𓃠   = 10000100 11100001 01100000

                    let mut b1 = 0b_1000_0000;
                    b1 = b1 | ((buffer[0] << 4) & 0b_0111_0000);
                    b1 = b1 | ((buffer[1] >> 2) & 0b_0000_1111);
                    bytes.push(b1);
                    let mut b2 = 0b_1000_0000;
                    b2 = b2 | ((buffer[1] << 5) & 0b_0110_0000);
                    b2 = b2 | ((buffer[2] >> 1) & 0b_0001_1111);
                    bytes.push(b2);
                    let mut b3 = 0b_0000_0000;
                    b3 = b3 | ((buffer[2] << 6) & 0b_0100_0000);
                    b3 = b3 | (buffer[3] & 0b_0011_1111);
                    bytes.push(b3);
                }
                _ => {
                    panic!("invalid utf-8 character with more than four bytes");
                }
            }
        }

        KimString {
            bytes,
        }
    }
}

impl Into<String> for KimString {

    fn into(self) -> String {
        let mut acc = 0u32;
        let mut result = String::with_capacity(self.bytes.len());

        for &char in self.bytes.iter() {
            acc = acc << 7 | (char & 0b_0111_1111) as u32;
            if char.leading_ones() == 0 {
                result.push(unsafe { char::from_u32_unchecked(acc) });
                acc = 0;
            }
        }

        result
    }
}

impl KimString {

    /// Returns the length of `self`.
    ///
    /// This length is in bytes, not [`char`]s or graphemes. In other words,
    /// it might not be what a human considers the length of the string.
    ///
    /// [`char`]: prim@char
    ///
    /// # Examples
    ///
    /// ```
    /// use kim_rs::KimString;
    /// 
    /// let len = KimString::from("ß").len();
    /// assert_eq!(2, len);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Converts a string slice to a byte slice.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::KimString;


    #[test]
    fn from_str_ascii() {
        let s = "c";
        let kim = KimString::from(s);
        assert_eq!(vec![0b_0110_0011], kim.bytes);

    }

    #[test]
    fn from_str_two_byte_coderange() {
        let s = "ß";
        let kim = KimString::from(s);
        assert_eq!(vec![0b_10000001, 0b_01011111], kim.bytes);
    }

    #[test]
    fn from_str_three_byte_coderange() {
        let s = "☃";
        let kim = KimString::from(s);
        assert_eq!(vec![0b_10000000, 0b_11001100, 0b_00000011], kim.bytes);

        let s = "♲";
        let kim = KimString::from(s);
        assert_eq!(vec![0b_10000000, 0b_11001100, 0b_01110010], kim.bytes);
    }

    #[test]
    fn from_str_four_byte_coderange() {
        let s = "𓂀";
        let kim = KimString::from(s);
        assert_eq!(vec![0b_10000100, 0b_11100001, 0b_00000000], kim.bytes);

        let s = "𓃠";
        let kim = KimString::from(s);
        assert_eq!(vec![0b_10000100, 0b_11100001, 0b_01100000], kim.bytes);
    }

    #[test]
    fn into_string() {
        let kim = KimString { bytes: vec![ 0b_10000100, 0b_11100001, 0b_00000000, 0b_10000001, 0b_01011111, 0b_01100001 ] };
        let result: String = kim.into();
        assert_eq!("𓂀ßa", result);
    }

    #[test]
    fn from_into() {
        let input = "𓂀𓃠𓅣𓂻𓂻𓂺𓁟𓂑𓃻𓇼𓊽𓂭𓎆𓍢𓏢𓐠";
        let kim = KimString::from(input);
        let output: String = kim.into();
        assert_eq!(input, output);
    }
}

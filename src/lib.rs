pub struct KimString {
    bytes: Vec<u8>,
}

impl From<&str> for KimString {

    fn from(s: &str) -> Self {
        let mut acc = Vec::with_capacity(s.len());
        for c in s.chars() {
            let mut buffer = [0u8; 4];
            c.encode_utf8(&mut buffer);

            match c.len_utf8() {
                1 => {
                    // ASCII mapped 1 to 1
                    acc.push(buffer[0] & 0b_0111_1111);
                }
                2 => {
                    // UTF-8 ß = 11000011 10011111
                    // KIM ß   = 10001101 01111000
                    let mut b1 = 0b_1000_0000;
                    b1 = b1 | (buffer[0] << 2);
                    b1 = b1 | ((buffer[1] >> 4) & 0b_0000_0011);
                    acc.push(b1);
                    let mut b2 = 0b_0111_1000;
                    b2 = b2 & (buffer[1] << 2);
                    acc.push(b2);
                }
                3 => {
                    // UTF-8 ☃ = 11100010 10011000 10000011
                    // KIM ☃   = 10010011 10000000 01100000
                    let mut b1 = 0b_1000_0000;
                    b1 = b1 | (buffer[0] << 3);
                    b1 = b1 | ((buffer[1] >> 3) & 0b_0000_0111);
                    acc.push(b1);
                    let mut b2 = 0b_1000_0000;
                    b2 = b2 | (buffer[1] << 4);
                    b2 = b2 | ((buffer[2] >> 2) & 0b_0000_1111);
                    acc.push(b2);
                    let mut b3 = 0b_0110_0000;
                    b3 = b3 & (buffer[2] << 5);
                    acc.push(b3);
                }
                4 => {
                    // UTF-8 𓂀 = 11110000 10010011 10000010 10000000
                    // KIM 𓂀   = 10000100 11100001 00000000
                    let mut b1 = 0b_1000_0000;
                    b1 = b1 | (buffer[0] << 4);
                    b1 = b1 | ((buffer[1] >> 2) & 0b_0000_1111);
                    acc.push(b1);
                    let mut b2 = 0b_1000_0000;
                    b2 = b2 | (buffer[1] << 5);
                    b2 = b2 | ((buffer[2] >> 1) & 0b_0001_1111);
                    acc.push(b2);
                    let mut b3 = 0b_0100_0000;
                    b3 = b3 & (buffer[2] << 6);
                    b3 = b3 | (buffer[3] & 0b_0111_1111);
                    acc.push(b3);
                }
                _ => {
                    panic!("invalid utf-8 character with more than four bytes");
                }
            }
        }

        KimString {
            bytes: acc,
        }
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
}

#[cfg(test)]
mod tests {
    use crate::KimString;


    #[test]
    fn from_str() {
        let s = "cat";
        let kim = KimString::from(s);
        println!("{}", kim.bytes.iter().map(|b| format!("{:08b} ", b)).collect::<String>());

        let s = "☃";
        let kim = KimString::from(s);
        println!("{}", kim.bytes.iter().map(|b| format!("{:08b} ", b)).collect::<String>());

        let s = "𓂀";
        let kim = KimString::from(s);
        println!("{}", kim.bytes.iter().map(|b| format!("{:08b} ", b)).collect::<String>());
    }
}

use core::fmt;

#[cfg(feature = "serde")]
extern crate serde;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct StaticString<const L: usize> {
    pub content: [u8; L],
}

impl<const L: usize> StaticString<L> {
    pub const fn empty() -> Self {
        Self { content: [0x0; L] }
    }

    pub fn new(str: &str) -> Self {
        let mut a = [0x0; L];
        a[..str.len().min(L)].copy_from_slice(&str.as_bytes()[..str.len().min(L)]);
        Self { content: a }
    }

    pub fn from_slice(src: &[u8]) -> Self {
        let srclen = src.len();
        let len = if L < srclen { L } else { srclen };
        let mut a = Self::empty();
        a.content[..len].copy_from_slice(&src[..len]);
        a
    }

    fn len_of_int(mut val: i32) -> usize {
        let neg_sign = val < 0;
        val = val.abs();
        let mut num_digits = 1;
        let mut comp = 10;
        while comp - 1 < val {
            comp *= 10;
            num_digits += 1;
        }
        if neg_sign {
            num_digits += 1;
        }
        assert_ne!(num_digits, 0);
        num_digits
    }

    pub fn from_int(mut val: i32) -> Self {
        let digits = Self::len_of_int(val.abs());
        let neg = val < 0;
        let offs = if neg { 1 } else { 0 };
        let mut s = Self::empty();
        val = val.abs();
        if neg {
            s.set_char(0, b'-');
        }
        if digits + offs > L && L <= 2 + offs {
            s = Self::new("###");
        } else if digits + offs > L {
            s.set_char(
                offs,
                0x30 + ((val / 10_i32.pow(digits as u32 - 2) + 5) / 10) as u8,
            );
            s.set_char(1 + offs, b'e');
            s.set_char(2 + offs, 0x30 + digits as u8 - 1);
        } else {
            for i in (0..digits).rev() {
                s.set_char(i + offs, 0x30 + (val % 10) as u8);
                val /= 10;
            }
        }
        s
    }

    pub fn set_char(&mut self, idx: usize, char: u8) {
        if idx < self.content.len() {
            self.content[idx] = char;
        }
    }

    pub fn str(&self) -> &str {
        str::from_utf8(&self.content[0..self.len()]).unwrap_or_default()
    }

    pub fn bytes(self) -> [u8; L] {
        self.content
    }

    pub fn len(self) -> usize {
        let mut len = 0;
        while len < L && self.content[len] != 0 {
            len += 1;
        }
        len
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
}

impl<const L: usize> fmt::Debug for StaticString<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("\"{}\"", self.str()))
    }
}

impl<const L: usize> fmt::Display for StaticString<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.str())
    }
}

impl<const L: usize> Default for StaticString<L> {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(feature = "serde")]
impl<const L: usize> serde::Serialize for StaticString<L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.str())
    }
}

#[cfg(feature = "serde")]
impl<'de, const L: usize> serde::Deserialize<'de> for StaticString<L> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StaticStringVisitor<const L: usize>;

        impl<'de, const L: usize> serde::de::Visitor<'de> for StaticStringVisitor<L> {
            type Value = StaticString<L>;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "a string of exactly {} bytes", L)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(StaticString::new(v))
            }
        }

        deserializer.deserialize_str(StaticStringVisitor::<L>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_8() {
        let a = StaticString::<8>::new("");
        let b = StaticString::<8>::new("abc");
        let c = StaticString::<8>::new("abcdefgh");
        let d = StaticString::<8>::new("lmnopqrstuvw");

        assert_eq!(a.str(), "");
        assert_eq!(b.str(), "abc");
        assert_eq!(c.str(), "abcdefgh");
        assert_eq!(d.str(), "lmnopqrs");
    }

    #[test]
    fn str_from_slice() {
        let slice_a = [b'H', b'e', b'l', b'l', b'o'];
        let slice_b = [
            b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd', b'!',
        ];
        let slice_c = [b'G', b'r', b'e', b'e', b't', b'i', b'n', b'g'];
        let slice_d = [];
        let a = StaticString::<8>::from_slice(&slice_a);
        let b = StaticString::<8>::from_slice(&slice_b);
        let c = StaticString::<8>::from_slice(&slice_c);
        let d = StaticString::<8>::from_slice(&slice_d);

        assert_eq!(a.str(), "Hello");
        assert_eq!(b.str(), "Hello Wo");
        assert_eq!(c.str(), "Greeting");
        assert_eq!(d.str(), "");
    }

    #[test]
    fn num_digits() {
        assert_eq!(StaticString::<0>::len_of_int(0), 1);
        assert_eq!(StaticString::<0>::len_of_int(1), 1);
        assert_eq!(StaticString::<0>::len_of_int(-1), 2);
        assert_eq!(StaticString::<0>::len_of_int(-9), 2);
        assert_eq!(StaticString::<0>::len_of_int(10), 2);
        assert_eq!(StaticString::<0>::len_of_int(99), 2);
        assert_eq!(StaticString::<0>::len_of_int(100), 3);
        assert_eq!(StaticString::<0>::len_of_int(999), 3);
        assert_eq!(StaticString::<0>::len_of_int(1000), 4);
        assert_eq!(StaticString::<0>::len_of_int(9999), 4);
        assert_eq!(StaticString::<0>::len_of_int(-1234), 5);
    }

    #[test]
    fn format_ints() {
        assert_eq!(StaticString::<8>::from_int(0).str(), "0");
        assert_eq!(StaticString::<8>::from_int(1).str(), "1");
        assert_eq!(StaticString::<8>::from_int(-1).str(), "-1");
        assert_eq!(StaticString::<8>::from_int(192).str(), "192");
        assert_eq!(StaticString::<1>::from_int(1234).str(), "#");
        assert_eq!(StaticString::<2>::from_int(1234).str(), "##");
        assert_eq!(StaticString::<3>::from_int(1234).str(), "1e3");
        assert_eq!(StaticString::<4>::from_int(1234).str(), "1234");
        assert_eq!(StaticString::<1>::from_int(-1234).str(), "#");
        assert_eq!(StaticString::<2>::from_int(-1234).str(), "##");
        assert_eq!(StaticString::<3>::from_int(-1234).str(), "###");
        assert_eq!(StaticString::<4>::from_int(-1234).str(), "-1e3");
        assert_eq!(StaticString::<5>::from_int(-1234).str(), "-1234");
        assert_eq!(StaticString::<8>::from_int(123456789).str(), "1e8");
        assert_eq!(StaticString::<8>::from_int(187654321).str(), "2e8");
        assert_eq!(StaticString::<16>::from_int(123456789).str(), "123456789");
    }
}

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

    fn copy_from<const N: usize>(
        &mut self,
        str: StaticString<N>,
        start: usize,
        end: usize,
        dest: usize,
    ) {
        if end > N || start > end || dest >= L || (end - start) + dest > L {
            return;
        }
        self.content[dest..dest + (end - start)].copy_from_slice(&str.content[start..end]);
    }

    pub fn insert_replace<const N: usize>(&mut self, str: StaticString<N>, idx: usize) {
        if idx > L {
            return;
        }
        let strlen = str.len();
        self.copy_from(
            str,
            0,
            if self.len() + strlen > L {
                L - self.len()
            } else {
                strlen
            },
            idx,
        );
    }

    pub fn append<const N: usize>(&mut self, str: StaticString<N>) {
        self.insert_replace(str, self.len());
    }

    pub fn append_char(&mut self, char: u8) {
        self.set_char(self.len(), char);
    }

    pub fn clear(&mut self) {
        *self = Self::empty();
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
        num_digits
    }

    fn len_of_fract(mut val: core::primitive::f32) -> usize {
        const EPS: f32 = core::primitive::f32::EPSILON;
        if val < EPS {
            return 1;
        }
        let mut num_digits = 0;
        while ((val as u32) as f32 - val).abs() >= EPS {
            val *= 10.0;
            num_digits += 1;
        }
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

    pub fn from_float<T: core::convert::Into<core::primitive::f32>>(val: T) -> Self {
        fn units_place(val: f32) -> u8 {
            (val as u32 % 10) as u8
        }

        extern crate std;
        let mut value: core::primitive::f32 = val.into();
        let neg = value < 0.0;
        let offs = if neg { 1 } else { 0 };
        let int = value.abs() as i32;
        let int_digits = Self::len_of_int(int);
        value = value.abs();
        let frac = value - (int as f32);
        let frac_digits = Self::len_of_fract(frac);
        if offs + int_digits > L {
            return Self::new("########");
        }

        let int_s = Self::from_int(int);
        let mut mul = 10.0;
        let mut s = Self::empty();
        if neg {
            s.append_char(b'-');
        }
        s.append(int_s);
        if offs + int_digits + 1 >= L {
            return s;
        }
        s.append_char(b'.');
        for _ in 0..frac_digits {
            s.append_char(0x30 + units_place(frac * mul));
            mul *= 10.0;
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
    type S<const L: usize> = StaticString<L>;

    #[test]
    fn str_8() {
        let a = S::<8>::new("");
        let b = S::<8>::new("abc");
        let c = S::<8>::new("abcdefgh");
        let d = S::<8>::new("lmnopqrstuvw");

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
        let a = S::<8>::from_slice(&slice_a);
        let b = S::<8>::from_slice(&slice_b);
        let c = S::<8>::from_slice(&slice_c);
        let d = S::<8>::from_slice(&slice_d);

        assert_eq!(a.str(), "Hello");
        assert_eq!(b.str(), "Hello Wo");
        assert_eq!(c.str(), "Greeting");
        assert_eq!(d.str(), "");
    }

    #[test]
    fn append() {
        let q = S::<4>::new("xyz");
        let mut s = S::<4>::empty();
        let mut ss = S::<8>::new("abcd");
        s.append(ss);
        assert_eq!(s.str(), "abcd");
        ss.append(s);
        assert_eq!(ss.str(), "abcdabcd");
        s.clear();
        ss.clear();
        s.append(q);
        assert_eq!(s.str(), "xyz");
        s.append(q);
        assert_eq!(s.str(), "xyzx");
    }

    #[test]
    fn append_char() {
        let mut s = S::<4>::empty();
        s.append_char(b'a');
        assert_eq!(s.str(), "a");
        s.append_char(b'b');
        s.append_char(b'c');
        s.append_char(b'd');
        assert_eq!(s.str(), "abcd");
        s.append_char(b'e');
        assert_eq!(s.str(), "abcd");
    }

    #[test]
    fn num_digits() {
        assert_eq!(S::<0>::len_of_int(0), 1);
        assert_eq!(S::<0>::len_of_int(1), 1);
        assert_eq!(S::<0>::len_of_int(-1), 2);
        assert_eq!(S::<0>::len_of_int(-9), 2);
        assert_eq!(S::<0>::len_of_int(10), 2);
        assert_eq!(S::<0>::len_of_int(99), 2);
        assert_eq!(S::<0>::len_of_int(100), 3);
        assert_eq!(S::<0>::len_of_int(999), 3);
        assert_eq!(S::<0>::len_of_int(1000), 4);
        assert_eq!(S::<0>::len_of_int(9999), 4);
        assert_eq!(S::<0>::len_of_int(-1234), 5);
    }

    #[test]
    fn format_ints() {
        assert_eq!(S::<8>::from_int(0).str(), "0");
        assert_eq!(S::<8>::from_int(1).str(), "1");
        assert_eq!(S::<8>::from_int(-1).str(), "-1");
        assert_eq!(S::<8>::from_int(192).str(), "192");
        assert_eq!(S::<1>::from_int(1234).str(), "#");
        assert_eq!(S::<2>::from_int(1234).str(), "##");
        assert_eq!(S::<3>::from_int(1234).str(), "1e3");
        assert_eq!(S::<4>::from_int(1234).str(), "1234");
        assert_eq!(S::<1>::from_int(-1234).str(), "#");
        assert_eq!(S::<2>::from_int(-1234).str(), "##");
        assert_eq!(S::<3>::from_int(-1234).str(), "###");
        assert_eq!(S::<4>::from_int(-1234).str(), "-1e3");
        assert_eq!(S::<5>::from_int(-1234).str(), "-1234");
        assert_eq!(S::<8>::from_int(123456789).str(), "1e8");
        assert_eq!(S::<8>::from_int(187654321).str(), "2e8");
        assert_eq!(S::<16>::from_int(123456789).str(), "123456789");
    }

    #[test]
    fn format_floats() {
        assert_eq!(S::<8>::from_float(0.0).str(), "0.0");
        assert_eq!(S::<8>::from_float(0.1).str(), "0.1");
        assert_eq!(S::<8>::from_float(0.9).str(), "0.9");
        assert_eq!(S::<8>::from_float(-0.1234).str(), "-0.1234");
        assert_eq!(S::<8>::from_float(-0.1234567).str(), "-0.12345");
        assert_eq!(S::<8>::from_float(-0.1234511).str(), "-0.12345");
        assert_eq!(S::<8>::from_float(0.1234561).str(), "0.123456");
        assert_eq!(S::<8>::from_float(0.1234569).str(), "0.123456");
        assert_eq!(S::<3>::from_float(-123.456).str(), "###");
        assert_eq!(S::<4>::from_float(-123.456).str(), "-123");
        assert_eq!(S::<5>::from_float(-123.456).str(), "-123");
        assert_eq!(S::<6>::from_float(-123.456).str(), "-123.4");
        assert_eq!(S::<7>::from_float(-123.456).str(), "-123.45");
        assert_eq!(S::<3>::from_float(123.456).str(), "123");
        assert_eq!(S::<4>::from_float(123.456).str(), "123");
        assert_eq!(S::<5>::from_float(123.456).str(), "123.4");
        assert_eq!(S::<6>::from_float(123.456).str(), "123.45");
        assert_eq!(S::<7>::from_float(123.456).str(), "123.456");
        assert_eq!(S::<8>::from_float(1.0).str(), "1.0");
        assert_eq!(S::<8>::from_float(-1.0).str(), "-1.0");
        assert_eq!(S::<8>::from_float(192.0).str(), "192.0");
        assert_eq!(S::<1>::from_float(1234.0).str(), "#");
        assert_eq!(S::<2>::from_float(1234.0).str(), "##");
        assert_eq!(S::<3>::from_float(1234.0).str(), "###");
        assert_eq!(S::<4>::from_float(1234.0).str(), "1234");
        assert_eq!(S::<1>::from_float(-1234.0).str(), "#");
        assert_eq!(S::<2>::from_float(-1234.0).str(), "##");
        assert_eq!(S::<3>::from_float(-1234.0).str(), "###");
        assert_eq!(S::<4>::from_float(-1234.0).str(), "####");
        assert_eq!(S::<5>::from_float(-1234.0).str(), "-1234");
    }
}

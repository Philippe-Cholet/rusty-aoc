/// Efficiently mimics the bytes representation of `u64` numbers on the stack,
/// especially sequentially with the `increment` method.
///
/// ```ignore
/// # use utils::U64Ascii;
/// fn job<T: AsRef<[u8]>>(data: T) -> usize {
///     let bytes = data.as_ref();
///     // ...
///     bytes.len()
/// }
///
/// let mut n_bytes = U64Ascii::default(); // zero
/// for n in 1..=129 {
///     n_bytes.increment();
///     assert_eq!(job(&n_bytes), job(&n.to_string()));
/// }
/// n_bytes.mul10();
/// assert_eq!(u64::from(&n_bytes), 1290);
/// n_bytes.div10();
/// assert_eq!(u64::from(&n_bytes), 129);
/// assert_eq!(u64::from(&U64Ascii::from(28198)), 28198);
/// assert_eq!(U64Ascii::from(181_949).to_string(), 181_949.to_string());
/// ```
///
/// It can represent bigger numbers than `u64::MAX`, up to `99_999_999_999_999_999_999`
/// but it obviously can not convert them back to `u64`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U64Ascii {
    index: usize,
    bytes: [u8; 20],
}

impl Default for U64Ascii {
    /// Zero.
    #[inline]
    fn default() -> Self {
        Self::MIN
    }
}

impl AsRef<[u8]> for U64Ascii {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.bytes[self.index..]
    }
}

impl From<u64> for U64Ascii {
    fn from(mut value: u64) -> Self {
        let mut result = Self::default();
        loop {
            result.bytes[result.index] += (value % 10) as u8;
            value /= 10;
            if value == 0 {
                break;
            }
            result.index -= 1;
        }
        result
    }
}

impl From<&U64Ascii> for u64 {
    fn from(value: &U64Ascii) -> Self {
        value
            .as_ref()
            .iter()
            .fold(0, |res, digit| res * 10 + Self::from(*digit - b'0'))
    }
}

impl std::fmt::Display for U64Ascii {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_str(&String::from_utf8(self.as_ref().to_vec()).unwrap())
        f.write_str(&String::from_utf8_lossy(self.as_ref()))
    }
}

impl U64Ascii {
    /// Represents `u64::MIN` (zero).
    pub const MIN: Self = Self {
        index: 19,
        bytes: [b'0'; 20],
    };

    /// Represents `u64::MAX`.
    pub const MAX: Self = Self {
        index: 0,
        bytes: *b"18446744073709551615",
    };

    /// `+= 1`
    pub fn increment(&mut self) {
        let res = self
            .bytes
            .iter_mut()
            .rposition(|d| {
                let nine = d == &b'9';
                if nine {
                    *d = b'0';
                }
                !nine
            })
            .map(|idx| {
                self.bytes[idx] += 1;
                self.index = self.index.min(idx);
            });
        debug_assert!(res.is_some(), "attempt to increment with overflow");
    }

    /// `*= 10`
    pub fn mul10(&mut self) -> bool {
        debug_assert_eq!(self.bytes[0], b'0', "attempt to mul by 10 with overflow");
        if self.as_ref() == b"0" {
            return true; // zero, nothing to do
        }
        self.index -= 1;
        self.bytes[self.index..].rotate_left(1);
        true
    }

    /// `/= 10`
    pub fn div10(&mut self) {
        self.bytes[0] = b'0';
        self.bytes[self.index..].rotate_right(1);
        self.index = (self.index + 1).min(19);
    }
    // For anything more complex than those three operations, we probably should
    // - convert to `u64`
    // - do the operation on the `u64`
    // - convert back from `u64`.
}

#[cfg(test)]
mod tests {
    use super::U64Ascii;

    macro_rules! eq {
        ($asc:ident, $n:ident, $msg:literal) => {
            assert_eq!(
                (&$asc).as_ref(),
                $n.to_string().as_bytes(),
                "{} as_ref {}",
                $msg,
                $n
            );
            assert_eq!(
                (&$asc).to_string(),
                $n.to_string(),
                "{} to_string {}",
                $msg,
                $n
            );
            assert_eq!(&$asc, &U64Ascii::from($n), "{} from u64 {}", $msg, $n);
            assert_eq!(u64::from(&$asc), $n, "{} into u64 {}", $msg, $n);
        };
    }

    #[test]
    #[ignore]
    fn first_hundred() {
        let mut asc = U64Ascii::default();
        for mut n in 0..=100 {
            eq!(asc, n, "basic");
            asc.increment();
            n += 1;
            asc.mul10();
            n *= 10;
            eq!(asc, n, "mul10");
            asc.div10();
            n /= 10;
            eq!(asc, n, "div10");
        }
    }

    #[test]
    #[ignore]
    fn max() {
        let max = u64::MAX;
        let mut asc = U64Ascii::from(max);
        assert_eq!(asc, U64Ascii::MAX, "MAX");
        eq!(asc, max, "max");
        asc.increment();
    }

    #[test]
    #[ignore]
    #[cfg_attr(debug_assertions, should_panic = "attempt to add with overflow")]
    #[cfg_attr(not(debug_assertions), should_panic = "u64 saturated")]
    fn incremented_max_into_u64() {
        let mut asc = U64Ascii::MAX;
        asc.increment();
        assert!(u64::from(&asc) != 0, "u64 saturated");
    }
}

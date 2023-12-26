use anyhow::{Context, Error, Result};

/// Convert a char representing a decimal digit to any type that can come from `u32`.
pub fn char10<T>(ch: char) -> Result<T>
where
    T: TryFrom<u32>,
    T::Error: Into<Error>,
{
    let dec = ch.to_digit(10).context("Not decimal")?;
    T::try_from(dec).map_err(Into::into)
}

/// Convert a char representing a hexadecimal digit to any type that can come from `u32`.
pub fn char16<T>(ch: char) -> Result<T>
where
    T: TryFrom<u32>,
    T::Error: Into<Error>,
{
    let hex = ch.to_digit(16).context("Not hexadecimal")?;
    T::try_from(hex).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_to_grid;

    #[test]
    #[ignore]
    fn test_char10() -> Result<()> {
        for (dec, ch) in "0123456789".chars().enumerate() {
            assert_eq!(char10::<u8>(ch)?, dec as u8);
            assert_eq!(char10::<i8>(ch)?, dec as i8);
            assert_eq!(char10::<u16>(ch)?, dec as u16);
            assert_eq!(char10::<i16>(ch)?, dec as i16);
            assert_eq!(char10::<i32>(ch)?, dec as i32);
            assert_eq!(char10::<u32>(ch)?, dec as u32);
            assert_eq!(char10::<i64>(ch)?, dec as i64);
            assert_eq!(char10::<u64>(ch)?, dec as u64);
            assert_eq!(char10::<i128>(ch)?, dec as i128);
            assert_eq!(char10::<u128>(ch)?, dec as u128);
            assert_eq!(char10::<usize>(ch)?, dec as usize);
            assert_eq!(char10::<isize>(ch)?, dec as isize);
            // As long as it can infer the type.
            let _x: Vec<i32> = vec![char10(ch)?];
            let _x: Vec<u32> = vec![char10(ch)?];
            let _x: Vec<usize> = vec![char10(ch)?];
            let _x: Vec<u8> = vec![char10(ch)?];
            let _x: Vec<i8> = vec![char10(ch)?];
        }
        let input = "123\n456\n789\n";
        let _grid = parse_to_grid(input.lines(), char10::<i8>)?;
        let grid = parse_to_grid(input.lines(), char10)?;
        let _n: u32 = grid[0][0];
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_char16() -> Result<()> {
        for (hex, ch) in "0123456789abcdef".chars().enumerate() {
            assert_eq!(char16::<u8>(ch)?, hex as u8);
            assert_eq!(char16::<i8>(ch)?, hex as i8);
            assert_eq!(char16::<u16>(ch)?, hex as u16);
            assert_eq!(char16::<i16>(ch)?, hex as i16);
            assert_eq!(char16::<i32>(ch)?, hex as i32);
            assert_eq!(char16::<u32>(ch)?, hex as u32);
            assert_eq!(char16::<i64>(ch)?, hex as i64);
            assert_eq!(char16::<u64>(ch)?, hex as u64);
            assert_eq!(char16::<i128>(ch)?, hex as i128);
            assert_eq!(char16::<u128>(ch)?, hex as u128);
            assert_eq!(char16::<usize>(ch)?, hex as usize);
            assert_eq!(char16::<isize>(ch)?, hex as isize);
            // As long as it can infer the type.
            let _x: Vec<i32> = vec![char16(ch)?];
            let _x: Vec<u32> = vec![char16(ch)?];
            let _x: Vec<usize> = vec![char16(ch)?];
            let _x: Vec<u8> = vec![char16(ch)?];
            let _x: Vec<i8> = vec![char16(ch)?];
        }
        let input = "0123\n4567\n89ab\ncdef\n";
        let _grid = parse_to_grid(input.lines(), char16::<i8>)?;
        let grid = parse_to_grid(input.lines(), char16)?;
        let _n: u32 = grid[0][0];
        Ok(())
    }
}

use common::prelude::*;
use crate::utils::U64Ascii;

/// The Ideal Stocking Stuffer
pub fn solver(part: Part, input: &str) -> Result<u64> {
    let max3 = [0, 0, part.value(0x0F, 0)];
    let mut context = md5::Context::new();
    context.consume(input.trim_end());
    let mut nb = U64Ascii::default();
    loop {
        let mut ctx = context.clone();
        nb.increment();
        ctx.consume(&nb);
        let digest = ctx.compute();
        if &digest.0[..3] <= &max3 {
            return Ok(u64::from(&nb));
        }
    }
}

test_solver! {
    "abcdef" => 609043,
    include_input!(15 04) => (117946, 3938038),
}

use std::io::Read;

use crate::types::AnyResult;

fn ascii_to_dec(ascii: u8) -> u8 {
    ascii - 48_u8
}

pub fn read_u32(max: u32) -> AnyResult<u32> {
    let mut stdin = std::io::stdin();

    let min_buf_size = (max as f32).log10().ceil() as usize;

    let mut input_buf = vec![0; min_buf_size];

    stdin.read(&mut input_buf[..])?;

    let output = input_buf.iter().rev().enumerate().fold(
        Ok(0_u32),
        |acc: AnyResult<u32>, (ind, next)| {
            let (num, did_overflow) =
                acc?.overflowing_add((ascii_to_dec(*next) as u32) * 10_i32.pow(ind as u32) as u32);

            match did_overflow {
                true => Err("Addition overflow, input exceeds max allowed".into()),
                false => Ok(num),
            }
        },
    )?;

    if output > max {
        Err("Output exceeded maximum".into())
    } else {
        Ok(output)
    }
}

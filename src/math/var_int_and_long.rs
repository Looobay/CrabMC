pub fn write_var_int(mut value: i32) -> Vec<u8> {
    let mut buffer = Vec::new();
    while (value & !0x7F) != 0 {
        buffer.push(((value & 0x7F) | 0x80) as u8);
        value >>= 7;
    }
    buffer.push((value & 0x7F) as u8);
    buffer
}

pub fn read_var_int(bytes: &[u8]) -> Result<i32, &'static str> {
    let mut value: i32 = 0;
    let mut position: usize = 0;
    let mut byte;

    loop {
        if position >= bytes.len() {
            return Err("Byte array is too short to be a valid VarInt");
        }
        byte = bytes[position] as i32;
        value |= (byte & 0x7F) << (7 * position);
        position += 1;

        if (byte & 0x80) == 0 {
            break;
        }
    }

    Ok(value)
}

// remove the "_" when we will use this function in future.
pub fn _write_var_long(mut value: i64) -> Vec<u8> {
    let mut buffer = Vec::new();
    while (value & !0x7F) != 0 {
        buffer.push(((value & 0x7F) | 0x80) as u8);
        value >>= 7;
    }
    buffer.push((value & 0x7F) as u8);
    buffer
}

// remove the "_" when we will use this function in future.
pub fn _read_var_long(bytes: &[u8]) -> Result<(i64, usize), &'static str> {
    let mut value: i64 = 0;
    let mut position: usize = 0;
    let mut byte;

    loop {
        if position >= bytes.len() {
            return Err("Byte array is too short to be a valid VarLong");
        }
        byte = bytes[position] as i64;
        value |= (byte & 0x7F) << (7 * position);
        position += 1;

        if (byte & 0x80) == 0 {
            break;
        }
    }

    Ok((value, position))
}

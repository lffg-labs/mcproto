use bytes::{Buf, BufMut};

const MSB_MASK: u8 = 0b1000_0000;

pub fn encode(mut value: u32, dst: &mut dyn BufMut) {
    while 128 <= value {
        let component = (value as u8) | MSB_MASK;
        dst.put_u8(component);
        value >>= 7;
    }
    dst.put_u8(value as u8);
}

pub fn decode(src: &mut dyn Buf) -> u32 {
    let mut i = 0;
    let mut shift = 0;
    while {
        let component = src.get_u8();

        i |= ((component & !MSB_MASK) as u32) << shift;
        shift += 7;

        (component & MSB_MASK) != 0 // has next?
    } {}
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    static CASES: &[(i32, &[u8])] = &[
        (0, b"\x00"),
        (1, b"\x01"),
        (2, b"\x02"),
        (127, b"\x7f"),
        (128, b"\x80\x01"),
        (255, b"\xff\x01"),
        (767, b"\xff\x05"),
        (25565, b"\xdd\xc7\x01"),
        (2097151, b"\xff\xff\x7f"),
        (2147483647, b"\xff\xff\xff\xff\x07"),
        (-1, b"\xff\xff\xff\xff\x0f"),
        (-2147483648, b"\x80\x80\x80\x80\x08"),
    ];

    #[test]
    fn enc_dec() {
        use bytes::Buf;

        for &(decoded, encoded) in CASES {
            println!("testing {decoded:?} ({encoded:?})");
            let mut buf = vec![];

            encode(decoded as u32, &mut buf);
            assert_eq!(buf.as_slice().chunk(), encoded);

            let d = decode(&mut buf.as_slice()) as i32;
            assert_eq!(d, decoded);
        }
    }
}

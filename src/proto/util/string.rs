use bytes::{Buf, BufMut};
use eyre::{Context, Result};

use crate::proto::util::varint;

// TODO: Check `len` (add a length parameter?)
pub fn encode(value: &str, dst: &mut dyn BufMut) -> Result<()> {
    // TODO: Check `len`
    varint::encode(value.as_bytes().len() as u32, dst);
    dst.put_slice(value.as_bytes());
    Ok(())
}

// TODO: Check `len` (add a length parameter?)
pub fn decode(src: &mut dyn Buf) -> Result<String> {
    let size = varint::decode(src) as usize;
    let mut buf = vec![0; size]; // allocate string buffer
    src.copy_to_slice(&mut buf);
    String::from_utf8(buf).wrap_err("unable to decode string")
}

#[cfg(test)]
mod tests {
    use super::*;

    static CASES: &[(&str, &[u8])] = &[
        //
        ("hello", b"\x05hello"),
        ("", b"\x00"),
    ];

    #[test]
    fn enc_dec() {
        use bytes::Buf;

        for &(decoded, encoded) in CASES {
            println!("testing {decoded:?} ({encoded:?})");
            let mut buf = vec![];

            encode(decoded, &mut buf).unwrap();
            assert_eq!(buf.as_slice().chunk(), encoded);

            let d = decode(&mut buf.as_slice()).unwrap();
            assert_eq!(d, decoded);
        }
    }
}

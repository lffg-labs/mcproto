use bytes::{Buf, BufMut};
use eyre::{ensure, Result};

use crate::proto::{util::varint, Serde};

pub const PACKET_ID: i32 = 0x00;

pub struct Req {}

impl Serde for Req {
    fn encode(&self, dst: &mut dyn BufMut) -> Result<()> {
        varint::encode(PACKET_ID as u32, dst);
        Ok(())
    }

    fn decode(src: &mut dyn Buf) -> Result<Self>
    where
        Self: Sized,
    {
        let id = varint::decode(src) as i32;
        ensure!(
            id == PACKET_ID,
            "invalid packet id for server-bound status request"
        );
        Ok(Req {})
    }
}

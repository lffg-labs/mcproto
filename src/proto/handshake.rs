// packet handshaking::Handshake (id: 0x00) {
//     proto_version: varint,
//     server_addr: string (255),
//     server_port: ushort,
//     next_state: handshaking::NextState,
// }

// enum handshaking::NextState (repr: varint) {
//     Status (1),
//     Login (2),
//     Transfer (3),
// }

use bytes::{Buf, BufMut};
use eyre::{bail, ensure, Result};

use crate::proto::{
    util::{string, varint},
    Serde,
};

pub const PACKET_ID: i32 = 0x00;

pub struct Req {
    pub proto_version: i32,  // varint
    pub server_addr: String, // string(255)
    pub server_port: u16,    // ushort
    pub next_state: NextState,
}

impl Serde for Req {
    fn encode(&self, dst: &mut dyn BufMut) -> Result<()> {
        varint::encode(PACKET_ID as u32, dst);
        varint::encode(self.proto_version as u32, dst);
        string::encode(&self.server_addr, dst)?;
        dst.put_u16(self.server_port);
        self.next_state.encode(dst)?;
        Ok(())
    }

    fn decode(src: &mut dyn Buf) -> Result<Self>
    where
        Self: Sized,
    {
        let id = varint::decode(src) as i32;
        ensure!(
            id == PACKET_ID,
            "invalid packet id for server-bound handshake"
        );
        Ok(Req {
            proto_version: varint::decode(src) as i32,
            server_addr: string::decode(src)?,
            server_port: src.get_u16(),
            next_state: NextState::decode(src)?,
        })
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum NextState /* varint */ {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

impl Serde for NextState {
    fn encode(&self, dst: &mut dyn BufMut) -> Result<()> {
        varint::encode((*self as u8) as u32, dst);
        Ok(())
    }

    fn decode(src: &mut dyn Buf) -> Result<Self> {
        match varint::decode(src) {
            1 => Ok(NextState::Status),
            2 => Ok(NextState::Login),
            3 => Ok(NextState::Transfer),
            unknown => bail!("unknown state id: {unknown}"),
        }
    }
}

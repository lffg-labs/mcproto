use bytes::{Buf, BufMut};
use eyre::Result;

pub mod handshake;
pub mod status;

pub mod util;

pub trait Serde {
    fn encode(&self, dst: &mut dyn BufMut) -> Result<()>;

    fn decode(src: &mut dyn Buf) -> Result<Self>
    where
        Self: Sized;
}

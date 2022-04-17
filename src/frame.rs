
#[derive(Clone, Debug)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Vec<u8>),
    Array(Vec<Frame>),
}

impl Frame {
    pub(crate) fn get_frame() -> Result<Frame> {
        Ok(Frame::Simple("aze"))
    }
}


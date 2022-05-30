#[derive(Debug)]
pub struct SeamPixel {
    pub seam: u32,
    pub pointer: Option<(u32, u32)>,
}

impl SeamPixel {
    pub fn new(seam: u32, pointer: Option<(u32, u32)>) -> SeamPixel {
        SeamPixel {
            seam,
            pointer,
        }
    }
}
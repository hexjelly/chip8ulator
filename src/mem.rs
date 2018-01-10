pub struct Mem {
    bytes: Box<[u8]>,
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            bytes: vec![0; 4096].into(),
        }
    }
}

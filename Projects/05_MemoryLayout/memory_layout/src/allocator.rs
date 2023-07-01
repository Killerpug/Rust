

pub const MAX_SIZE:usize = 512;// Bytes

enum MemoryState {
    Full,
    Partial,
    Free,
}

#[derive(Clone)]
pub struct Memory {
    pub mem: Vec<u8>,
}



struct Node {
    id: u8,
    state: MemoryState,
}


impl Memory {
    pub fn new() -> Self {
        let mut phymem: Vec<u8> = Vec::with_capacity(MAX_SIZE);
        for _i in 0..MAX_SIZE {
            phymem.push(0xFF);
        }
        Self { mem: phymem,}
    }
    pub fn allocate(&mut self, bytes:usize) {
        
        for i in 0..bytes {
            self.mem[i] = 0x00;
        }
    }



}


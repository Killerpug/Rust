
pub const MAX_SIZE:usize = 1024; // Bytes
pub const MIN_SIZE:usize = 8;  // Bytes: id(x2), memory state, data(between 1 and (MAX_SIZE-7))

#[derive(Clone, PartialEq)]
enum MemoryState {
    Full,
    Partial,
    Free,
}

#[derive(Clone)]
pub struct Memory {
    pub mem: Vec<u8>,
    pub freenodes: Vec<Node>,
    pub usednodes: Vec<Node>
}


#[derive(Clone)]
pub struct Node {
    id: u16,            // identifies 2^16 different ordered slabs
    data_ptr: usize,
    state: MemoryState,
    size: usize,
}


impl Memory {
    pub fn new() -> Self {
        let mut phymem: Vec<u8> = Vec::with_capacity(MAX_SIZE);
        for _i in 0..MAX_SIZE {
            phymem.push(0xFF);
        }
        let node = Node {
            id: 1,
            data_ptr: 0,
            state: MemoryState::Free,
            size: MAX_SIZE,
        };
        let mut phyfreenodes = vec![node];
        let mut phyusednodes:Vec<Node> = Vec::with_capacity(1);
        
        Self { mem: phymem, freenodes:phyfreenodes, usednodes: phyusednodes}
    }

    pub fn allocate(&mut self, bytes:usize) -> u16 {    //returns address of available space
        let mut allocsize: usize = MAX_SIZE;

        for node in self.freenodes.iter_mut() {
            if node.state != MemoryState::Full {
                while bytes * 2 < allocsize {
                    allocsize >>= 1;         //reduce size until find a suitable one(at most 2x the requested size)
                }
                print!("{}\n",allocsize);
                for i in 0..allocsize 
                {
                    self.mem[node.data_ptr + i] = node.id as u8;
                }
                node.id += 1;
                node.data_ptr =  node.data_ptr + allocsize;
                node.size -= allocsize;
            } else {
                continue;
            }
        }
        return 65535;   // memory is full
    }
}




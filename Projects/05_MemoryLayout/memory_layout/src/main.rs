
mod allocator;

fn main() {
    let mut mem_map = allocator::Memory::new();
    mem_map.allocate(10);
    for n in 0..512 {
        print!("{:#0x} ", &mem_map.mem[n]);
    }

}

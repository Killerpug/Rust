
mod allocator;

fn main() {
    let mut mem_map = allocator::Memory::new();
    mem_map.allocate(10);
    mem_map.allocate(5);
    mem_map.allocate(3);
    mem_map.allocate(1);

    for n in 0..512 {
        print!("{:#0x} ", &mem_map.mem[n]);
    }

}

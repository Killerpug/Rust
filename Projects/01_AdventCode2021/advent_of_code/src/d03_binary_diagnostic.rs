// calculate the most common bit value on each position after all calls to get gamma rate.
// we receive n characters representing the presence or absence of the diagnostic bit.
//ex. 00100

const NUMBER_OF_BITS: usize = 12;
use std::error::Error;
use std::fs;
use std::fs::File;

/// DiagnosticCounter stores a counter for each diagnostic bits
#[derive(Debug)]
pub struct DiagnosticCounter {
    bitcounters:[u32; NUMBER_OF_BITS],
    pub gamma_rate: u32,
    pub epsilon_rate: u32,
    total_samples: u32,
}
impl DiagnosticCounter {
    pub fn new() -> Self {
        Self{
            bitcounters: [0; NUMBER_OF_BITS],
            gamma_rate: 0,
            epsilon_rate: 0,
            total_samples: 0,
        }
    }
}

pub fn calculate_diagnostic_counters() {
    let mut diag_count = DiagnosticCounter::new();
    let reader = fs::read_to_string("src/input/d03_input_binarydiag.txt").unwrap();
    for line in reader.lines() {
        let mut bits = u32::from_str_radix(line, 2).expect("not a binary number");
        for bit in  0..NUMBER_OF_BITS {
            diag_count.bitcounters[bit] += bits & 0x01;
            bits = bits >> 1;
        }
        diag_count.total_samples += 1;
    }
    print!("total bits in each position, out of: {} samples \n ", diag_count.total_samples);
    for pos in (0..12).rev() {
        print!("pos:{} counter: {}\n ", pos, diag_count.bitcounters[pos]);
        // finding most common value
        if diag_count.bitcounters[pos] > (diag_count.total_samples/2) {
            diag_count.gamma_rate += 1;
        } 
        diag_count.gamma_rate = diag_count.gamma_rate << 1;
        
    }
    diag_count.gamma_rate = diag_count.gamma_rate >> 1;
    diag_count.epsilon_rate = (!diag_count.gamma_rate) & 0xFFF;
    println!("gamma rate: {:b}, epsilon rate = {:b} \n",diag_count.gamma_rate, diag_count.epsilon_rate);
    print!("diag result = gamma * epsilon = {}\n", diag_count.gamma_rate * diag_count.epsilon_rate)
}

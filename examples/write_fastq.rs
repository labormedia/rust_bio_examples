use std::io;
use bio::io::fastq;

fn main() {
    let mut seed = 42;

    let nucleotides = [b'A', b'C', b'G', b'T'];
    
    let mut writer = fastq::Writer::new(io::stdout());
    
    for _ in 0..10 {
        let seq = (0..100).map(|_| {
            seed = ((seed ^ seed << 13) ^ seed >> 7) ^ seed << 17; // don't use this random generator
            nucleotides[seed % 4]
        }).collect::<Vec<u8>>();
    
        let qual = (0..100).map(|_| b'!').collect::<Vec<u8>>();
    
       writer.write("random", None, seq.as_slice(), qual.as_slice());
    }
}


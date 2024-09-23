// Import some modules
use bio::alphabets;
use bio::data_structures::bwt::{bwt, less, Occ};
use bio::data_structures::fmindex::{BackwardSearchResult, FMIndex, FMIndexable};
use bio::data_structures::suffix_array::suffix_array;
use bio::io::fastq;
use bio::io::fastq::FastqRead;
use std::io;

fn main() {
    // a given text
    let text = b"ACAGCTCGATCGGTA$";
    let pattern = b"ATCG";

    // Create an FM-Index for the given text.

    // instantiate an alphabet
    let alphabet = alphabets::dna::iupac_alphabet();
    // calculate a suffix array
    let sa = suffix_array(text);
    // calculate the Burrows-Wheeler-transform
    let bwt = bwt(text, &sa);
    // calculate the vectors less and Occ (occurrences)
    let less = less(&bwt, &alphabet);
    let occ = Occ::new(&bwt, 3, &alphabet);
    // set up FMIndex
    let fmindex = FMIndex::new(&bwt, &less, &occ);
    // do a backwards search for the pattern
    let interval = fmindex.backward_search(pattern.iter());
    let mut partial_match_len = 0;
    // get the locations where the pattern matched (completely in this case).
    let positions = match interval {
        BackwardSearchResult::Complete(saint) => saint.occ(&sa),
        BackwardSearchResult::Partial(saint, l) => {
            partial_match_len = l;
            saint.occ(&sa)
        }
        BackwardSearchResult::Absent => Vec::new(),
    };
    // Iterate over a FASTQ file, use the alphabet to validate read
    // sequences and search for exact matches in the FM-Index.

    // create FASTQ reader
    let mut reader = fastq::Reader::new(io::stdin());
    let mut record = fastq::Record::new();
    let mut partial_match_len = 0;
    reader.read(&mut record).expect("Failed to parse record");
    while !record.is_empty() {
        let check = record.check();
        if check.is_err() {
            panic!("I got a rubbish record!")
        }
        // obtain sequence
        let seq = record.seq();
        // check, whether seq is in the expected alphabet
        if alphabet.is_word(seq) {
            let interval = fmindex.backward_search(seq.iter());
            // get the positions where seq matched completely
            // or where the maximal matching suffix of seq occurred.
            let positions = match interval {
                BackwardSearchResult::Complete(saint) => saint.occ(&sa),
                BackwardSearchResult::Partial(saint, l) => {
                    partial_match_len = l;
                    saint.occ(&sa)
                }
                BackwardSearchResult::Absent => Vec::new(),
            };
        }
        reader.read(&mut record).expect("Failed to parse record");
    }
}

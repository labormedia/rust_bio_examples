# rust_bio_examples

This is a compilation of rust-bio implementations.

## Compile
```bash
cargo build --release
```

## Run random sequence example
```bash
cargo run --release --example write_fastq
```

## Pipe random sequence example to fm-index example (needs compilation).
```bash
./target/release/examples/write_fastq | ./target/release/examples/fm_index_fastq
```
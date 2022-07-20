# fxtools
A collection of fasta/fastq related tools that I've needed to write. 

# Installation
```bash
# from crates.io
cargo install fxtools

# from github
git clone https://github.com/noamteyssier/fxtools
cd fxtools
cargo install --path .
```

# Commands

## Unique
This is a command that will determine all unique sequences within a fastx file and split the records into either unique or duplicates.
It will also count the number of unique/duplicate sequences/duplicate records and report those.
By default all unique reads will be pushed to stdout unless piped to a file with the `-o` flag.
Nulled reads will not be reported by default but can be written to a filepath with the `-n` flag.

```bash
fxtools unique \
  -i <input_fastx> \
  -o <optional_output_file_for_unique> \
  -n <optional_output_file_for_null>
```

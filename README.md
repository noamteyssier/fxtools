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

## sgRNA Table
This command will create a table mapping sgRNA names to their parent gene. 
This works by parsing the header of each record and currently it expects the header to be as follows:
```bash
# {gene}_{auxilliary sgrna description}
```

The command requires an input fasta/q file and will by default write a sgrna-to-gene table to stdout.

You can pipe the output table to a file with the `-o` flag.

You can also choose to include each records sequence with the `-s` flag. 

You can also choose to reorder the columns to whatever format you'd like with the `-r` flag
and provide a 3 character string (i.e. `-r hsg` or `-r ghs`) representing the `[hH]eader`, 
`[sS]sequence`, and `[gG]ene`.

By default the table's delimiter is tabs, but you can specify a separate delimiter with the `-d` flag.

```bash
fxtools sgrna-table \
  -i <input_fastx> \
  -o <s2g.txt> \
  -s \
  -r ghs \
  -d <character delim>
```


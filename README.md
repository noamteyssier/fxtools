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

## Upper
This command will convert your input fastx into an output fastx with all nucleotides converted to their uppercase.
This will also validate to ensure there are no unexpected nucleotides found.
Default will write to stdout, but you can provide an output file with the `-o` flag.

```bash
fxtools upper \
  -i <input_fastx> \
  -o <output_fastx>
```

## Extract Variable
This command will extract the variable regions from an input fastx and write that variable regions to the output fastx. 

It was designed assuming that the sequences are all equal size and that they are prefixed and suffixed by a fairly static
nucleotide region (consider CRISPRi/a libraries with a constant adapter sequence on either side of a highly variable region).
This works by calculating the positional entropy across the nucleotides at each position, then applies a z-score threshold on
those entropies to determine a contiguous variable region which is then used as the bounds to write the output sequences.

Default will write to stdout, but you can provide an output file with the `-o` flag.
You can decide how many sequences to calculate the entropy on with the `-n` flag.
You can decide what z-score threshold to use for your data with the `-z` flag.

```
fxtools extract-variable \
  -i <input_fastx> \
  -o <output_fastx> \
  -n <number of sequences to use in fitting entropy [default: 5000]> \
  -z <zscore threshold to use [default: 1.]>
```

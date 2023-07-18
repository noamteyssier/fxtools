# [ `fxtools sgrna-table` ]

## Summary
This command will create a table mapping sgRNA names to their parent gene. 

### Expected Input
This works by parsing the header of each record and currently it expects the header to be as follows:
```bash
# {gene}_{auxilliary sgrna description}
```

### Parameters

The command requires an input fasta/q file and will by default write a sgrna-to-gene table to stdout.

You can pipe the output table to a file with the `-o` flag.

You can also choose to include each records sequence with the `-s` flag. 

You can also choose to reorder the columns to whatever format you'd like with the `-r` flag
and provide a 3 character string (i.e. `-r hsg` or `-r ghs`) representing the `[hH]eader`, 
`[sS]sequence`, and `[gG]ene`.

By default the table's delimiter is tabs, but you can specify a separate delimiter with the `-d` flag.

## Usage

```bash
fxtools sgrna-table \
  -i <input_fastx> \
  -o <s2g.txt> \
  -s \
  -r ghs \
  -d <character delim>
```


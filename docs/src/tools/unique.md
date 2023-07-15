# [ `fxtools unique` ]

This is a command that will determine all unique sequences within a fastx file and split the records into either unique or duplicates.
It will also count the number of unique/duplicate sequences/duplicate records and report those.

By default all unique reads will be pushed to stdout unless piped to a file with the `-o` flag.
Nulled reads will not be reported by default but can be written to a filepath with the `-n` flag.

## Usage

```bash
fxtools unique \
  -i <input_fastx> \
  -o <optional_output_file_for_unique> \
  -n <optional_output_file_for_null>
```

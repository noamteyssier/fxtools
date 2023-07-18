# [ `fxtools upper` ]

## Summary

This command will convert your input fastx into an output fastx with all nucleotides converted to their uppercase.
This will also validate to ensure there are no unexpected nucleotides found.

### Parameters

Default will write to stdout, but you can provide an output file with the `-o` flag.

## Usage

```bash
fxtools upper \
  -i <input_fastx> \
  -o <output_fastx>
```

# [ `fxtools fix` ]

## Summary

This command will replace all nucleotides not matching: `[ACTGNactgn]`
with the missing nucleotide `N`.

### Parameters

Default will write to stdout, but you can provide an output file with the `-o` flag.

## Usage

```bash
fxtools fix \
  -i <input_fastx> \
  -o <output_fastx>
```

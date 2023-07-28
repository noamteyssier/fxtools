# [ `fxtools fix` ]

## Summary

This command will replace all non-canonical nucleotides (`[ACTGNactgn]`)
with the missing nucleotide `N`.

### Parameters

Default will write to stdout, but you can provide an output file with the `-o` flag.

## Usage

```bash
fxtools upper \
  -i <input_fastx> \
  -o <output_fastx>
```

# [ `fxtools reverse` ]

## Summary

This command will convert your input `fastx` into an output `fastx`
with all nucleotide sequences (and associated quality scores) in
in reverse order.

Useful for `grep` for a sequence in `R2` or vice versa.

### Expected Input

This will reverse each of the sequences and potential quality scores.

This **will not** take the reverse complement of your sequences.

``` text
[sequence]
[sequence]
   ...
[sequence]
```

### Expected Output

``` text
[ecneuqes]
[ecneuqes]
   ...
[ecneuqes]
```

## Usage

``` bash
fxtools reverse -i <your_fwd.fq.gz>
```

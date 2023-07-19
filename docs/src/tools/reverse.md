# [ `fxtools reverse` ]

## Summary

This command will convert your input `fastx` into an output `fastx`
with all nucleotide sequences (and associated quality scores) in
in reverse order.

Useful for `grep` for a sequence in `R2` or vice versa.

### Expected Input

This will reverse complement each of the sequences and potential quality scores.

``` text
ACTG
GCTA
AAAA
```

### Expected Output

``` text
CAGT
TAGC
TTTT
```

## Usage

``` bash
fxtools reverse -i <your_fwd.fq.gz>
```

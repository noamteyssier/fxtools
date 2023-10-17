# [ `fxtools clip` ]

## Summary

This command will clip (or truncate) the records to only recover nucleotides within
a desired range.

### Expected Input

``` text
>AP2S1_a
ACTG
>AP2S1_b
ACTT
>AP2S2_a
CCCT
```

### Expected Output

```bash
# trims 1 nucleotide from the start and 1 nucleotide from the end
fxtools clip -s 1 -e 1
```

``` text
>AP2S1_a
CT
>AP2S1_b
CT
>AP2S2_a
CC
```

## Usage

``` bash
# left side clip (10 nucleotides)
fxtools clip -i <fastx> -s 10

# right side clip (10 nucleotides)
fxtools clip -i <fastx> -e 10

# left side clip (5 nucleotides) right side clip (15) nucleotides
fxtools clip -i <fastx> -s 5 -e 15

# clip everything outside of nucleotide range 10-20
fxtools clip -i <fastx> -r 10..20

# clip everything outside range 10-end (equivalent to -s 10)
fxtools clip -i <fastx> -r 10..

# clip everything outside range start-10
fxtools clip -i <fastx> -r ..10
```


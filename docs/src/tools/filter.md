# [ `fxtools filter` ]

## Summary

This command will filter your input `fastx` into an output `fastx`
and retrieve all the sequences or headers that match your input pattern.

This pattern is a regex compatible pattern, and can also be inverted with
the `-v` flag (like `grep -v`).

### Expected Input

This will reverse complement each of the sequences and potential quality scores.

``` text
>AP2S1_a
ACTG
>AP2S1_b
ACTT
>AP2S2_a
CCCT
```

### Expected Output

#### Filter on Sequence

``` bash
fxtools filter -i <fasta> -p "ACT"
```

``` text
>AP2S1_a
ACTG
>AP2S1_b
ACTT
```

#### Filter on Header

``` bash
fxtools filter -i <fasta> -p "_a" -H
```

``` text
>AP2S1_a
ACTG
>AP2S2_a
CCCT
```

#### Inverse Filter

``` bash
fxtools filter -i <fasta> -p "ACT" -v
```

``` text
>AP2S2_a
CCCT
```

## Usage

``` bash
# standard filtering (on sequence)
fxtools filter -i <fastx> -p <pattern>

# filtering on header
fxtools filter -i <fastx> -p <pattern> -H

# inverse filter (removing all records that match pattern)
fxtools filter -i <fastx> -p <pattern> -v

# inverse filter (removing all records that match pattern) on header
fxtools filter -i <fastx> -p <pattern> -v -H
```

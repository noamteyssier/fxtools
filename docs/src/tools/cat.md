# [ `fxtools cat` ]

## Summary

This command will concatenate multiple fastx files into a single stream.

### Expected Input

#### `sample_1.fa`

```text
>AP2S1_a
ACTG
>AP2S1_b
ACTT
>AP2S2_a
CCCT
```

#### `sample_2.fa`

```text
>NSD1_a
ACTG
>NSD1_b
ACTT
>NSD2_a
CCCT
```

### Expected Output

```text
>AP2S1_a
ACTG
>AP2S1_b
ACTT
>AP2S2_a
CCCT
>NSD1_a
ACTG
>NSD1_b
ACTT
>NSD2_a
CCCT
```

## Usage

```bash
# standard concatenation
fxtools cat -i <fastx> <...> <fastx>

# into pipeline
fxtools cat -i <fastx_1> <fastx_2> <fastx_3> | fxtools filter -p "ACT"
```

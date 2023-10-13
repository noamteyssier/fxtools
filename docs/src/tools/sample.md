# [ `fxtools sample` ]

## Summary

This command will randomly downsample the number of records in your input `fastx`
by some frequency `f`.

### Expected Input

```text
>AP2S1_a
ACTG
>AP2S1_b
ACTT
>AP2S2_a
CCCT
>AP2S2_b
CCCC
```

### Expected Output

With frequency 0.75.

```text
>AP2S1_a
ACTG
>AP2S2_a
CCCT
>AP2S2_b
CCCC
```

## Usage

```bash
# standard sampling 50% of records
fxtools sample -i <fastx> -f 0.5

# from pipeline subsampling 30% of records
fxtools filter -i <fastx> -p "ACTCGCG" | fxtools sample -f 0.3
```


# [ `fxtools count` ]

## Summary

This command will just count the number of records in your input `fastx`

### Expected Input

```text
>AP2S1_a
ACTG
>AP2S1_b
ACTT
>AP2S2_a
CCCT
```

### Expected Output

```text
3
```

## Usage

```bash
# standard counting
fxtools count -i <fastx>

# from pipeline
fxtools filter -i <fastx> -p "ACTCGCG" | fxtools count
```

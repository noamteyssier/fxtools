# [ `fxtools take` ]

## Summary

This command will take a certain number of records from your input `fastx`

### Expected Input

```text
>AP2S1_a
ACTG
>AP2S1_b
ACTT
>AP2S2_a
CCCT
```

### Expected Output (take 2)

```text
>AP2S1_a
ACTG
>AP2S1_b
ACTT
```

## Usage

```bash
# standard take (taking 3 records from the top)
fxtools take -i <fastx> -n 3

# standard take (skipping 2 and then taking 3 records)
fxtools take -i <fastx> -s 2 -n 3

# from pipeline
# filtering for a sequence pattern, and then taking the first 30 hits
fxtools filter -i <fastx> -p "ACTCGCG" | fxtools take -n 30
```

# [ `fxtools extract-variable` ]

## Summary

This command will extract the variable regions from an input `fastx` and write those variable regions to the output `fastx`. 

### Expected Input Sequences

It was designed assuming that the sequences are all equal size and that they are prefixed and suffixed by a fairly static
nucleotide region (consider CRISPRi/a libraries with a constant adapter sequence on either side of a highly variable region).

``` text
[prefix][variable][suffix]
[prefix][variable][suffix]
           ...
[prefix][variable][suffix]
```

### Expected Output Sequences

The output sequences will extract just the positions of the input sequence that have a higher entropy than
random chance.

``` text
[variable]
[variable]
   ...
[variable]
```

### How it Works

This works by calculating the positional entropy across the nucleotides at each position, then applies a z-score threshold on
those entropies to determine a contiguous variable region which is then used as the bounds to write the output sequences.

### Parameters

Default will write to stdout, but you can provide an output file with the `-o` flag.
You can decide how many sequences to calculate the entropy on with the `-n` flag.
You can decide what z-score threshold to use for your data with the `-z` flag.

> **Note:**
>
> The z-score threshold default is arbitrarily set.
> If you have a smaller number of sequences try to reduce the
> threshold to `0.5`, and see if that helps.

## Usage

```
fxtools extract-variable \
  -i <input_fastx> \
  -o <output_fastx> \
  -n <number of sequences to use in fitting entropy [default: 5000]> \
  -z <zscore threshold to use [default: 1.]>
```

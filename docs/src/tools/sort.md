# [ `fxtools sort` ]

This tool can be used to sort a `fastx` file or
a pair of equivalently sized `fastx` files based on
sequence.

If an `R2` is provided, then the default is to sort
on its sequences.

## Usage

``` bash
# Sort a single fastq
fxtools sort -i <your_file.fastq>

# Sort a paired-end fastq set by R2
fxtools sort -i <your_R1.fq.gz> -I <your_R2.fq.gz>

# Sort a paired-end fastq set by R1
fxtools sort -i <your_R1.fq.gz> -I <your_R2.fq.gz> --sort-by-r1
```

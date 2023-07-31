# [ `fxtools t2g` ]

## Summary

This tool can be used to parse a cDNA fasta file
and build a mapping of transcripts to genes which
is used in Kallisto.

You can decide to include the ensembl `gene_id`
or the `gene_name` which is the common symbol
of that gene.

## Usage

``` bash
# parse the t2g and write to stdout
fxtools t2g -i <your_seq.cdna.fasta.gz>

# parse the t2g and write to `t2g.txt`
fxtools t2g -i <your_seq.cdna.fasta.gz> -o t2g.txt

# parse the t2g and write the symbols instead of the gene_id
fxtools t2g -i <your_seq.cdna.fasta.gz> -s

# parse the t2g and include the gene_id version in the output
fxtools t2g -i <your_seq.cdna.fasta.gz> -s -d
```

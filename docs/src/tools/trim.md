# [ `fxtools trim` ]

This tool can be used to select and trim sequences
that contain a specified adapter sequence.

This can almost be thought of as a combination of `grep` 
and `sed`, where everything before the `grep` match is
removed.

By default the adapter sequence is kept (the adapter
will be a common prefix for all kept reads) but it
can also be trimmed away.

## Usage

``` bash
# trim away sequences prefixing the adapter
fxtools trim -i <your_seq.fq.gz> -a ACTTGGA

# trim away sequences prefixing the adapter + the adapter
fxtools trim -i <your_seq.fq.gz> -a ACTTGGA --trim-adapter
```

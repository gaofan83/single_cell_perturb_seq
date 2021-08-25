### This directory includes hashtag_tool executable to process single-cell hashtag data (tested on Ubuntu 18.04).
###
### USAGE:
 hashtag_tool [OPTIONS] <br/>
<br/>
 FLAGS: <br/>
    -h, --help       Prints help information <br/>
    -V, --version    Prints version information <br/>
<br/>
 OPTIONS: <br/>
        --R1 <Read1>                   R1 raw fastq file (doesn't work for *gz file) <br/>
        --R2 <Read2>                   R2 raw fastq file (doesn't work for *gz file) <br/>
        --bc_len <barcode_len>         the length of barcodes <br/>
        --bc <barcodes>                A list of cell barcodes <br/>
        --ft <features>                A list of hashtag barcodes <br/>
        --hashtag_len <hashtag_len>    the length of hashtag <br/>
        --umi_len <umi_len>            the length of UMI <br/>
<br/>
 
### Quick start:
`hashtag_tool --R1 R1.fastq --R2 R2.fastq --bc barcodes.txt --ft hashtag.txt --bc_len 16 --umi_len 12 --hashtag_len 15`

##### The output matrix file is called `ft_count.txt`

### This directory includes hashtag_tool executable file to process single-cell hashtag data (tested on Ubuntu 18.04).
####
#### USAGE:
####    hashtag_tool [OPTIONS]
####
#### FLAGS:
####    -h, --help       Prints help information
####    -V, --version    Prints version information
####
#### OPTIONS:
####        --R1 <Read1>                   R1 raw fastq file (doesn't work for *gz file)
####        --R2 <Read2>                   R2 raw fastq file (doesn't work for *gz file)
####        --bc_len <barcode_len>         the length of barcodes
####        --bc <barcodes>                A list of cell barcodes
####        --ft <features>                A list of hashtag barcodes
####        --hashtag_len <hashtag_len>    the length of hashtag
####        --umi_len <umi_len>            the length of UMI
####
#### Quick start:
`hashtag_tool --R1 R1.fastq --R2 R2.fastq --bc bc.txt --ft hash.txt --bc_len 16 --umi_len 12 --hashtag_len 15`


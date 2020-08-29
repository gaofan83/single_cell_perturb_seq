### This directory includes guiderna_tool executable to process single-cell dual-guide data (tested on Ubuntu 18.04).
###
### USAGE:
 guiderna_tool [OPTIONS] <br/>
<br/>
 FLAGS: <br/>
    -h, --help       Prints help information <br/>
    -V, --version    Prints version information <br/>
<br/>
 OPTIONS: <br/>
        --R1 <Read1>       R1 raw fastq file (doesn't work for *gz file) <br/>
        --R2 <Read2>       R2 raw fastq file (doesn't work for *gz file) <br/>
        --bc <barcodes>    A list of cell barcodes <br/>
        --gd1 <guides1>    List1 of sgRNA protospacer sequences that should be captured by Capture1 sequence <br/>
        --gd2 <guides2>    List2 of sgRNA protospacer sequences that should be captured by Capture2 sequence <br/>
<br/>
 
### Quick start:
`~/tools/guiderna_tool --R1 SE6383_SA69723_S3_L006_R1_001.fastq --R2 SE6383_SA69723_S3_L006_R2_001.fastq --bc barcodes_perturb_guide.tsv --gd1 guides1.txt --gd2 guides2.txt`

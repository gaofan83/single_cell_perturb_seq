### This directory includes guiderna_tool executable to process single-cell dual-guide data (tested on Ubuntu 18.04).
####
![Workflow](/guiderna/bioinformatics_pipeline.png)

#### USAGE:
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
 
#### Quick start:
`guiderna_tool --R1 R1.fastq --R2 R2.fastq --bc barcodes_perturb.tsv --gd1 guides1.txt --gd2 guides2.txt`
##### 
#### Demo run
#### 1) Download a demo guide RNA data from GEO database
`fastq-dump --split-files SRR13587911`
#####
#### 2) Download barcode mapping file from 10XG website https://github.com/10XGenomics/cellranger/blob/master/lib/python/cellranger/barcodes/translation/3M-february-2018.txt.gz
##### The cell barcodes for RNA and Feature are different on the same bead. That is why a mapping file is necessary to convert the barcode sequences between the two.  
`gunzip 3M-february-2018.txt.gz`
##### Note, in `3M-february-2018.txt` file, Column-1 is Feature cell barcodes, Column-2 is RNA cell barcodes.
#####
#### 3) Download guides1.txt, guides2.txt, scrna_barcodes.tsv from this directory https://github.com/gaofan83/single_cell_perturb_seq/tree/master/guiderna/demo.
##### `scrna_barcodes.tsv` includes a filtered list of mRNA cell barcodes from paired single-cell RNA-seq pre-processing results (e.g., processed with Cellranger pipeline); `guides1.txt` and `guides2.txt` include the guide RNA sequences.
#### 4) Run the following command lines in R console to convert RNA cell barcodes to Feature cell barcodes.
```
table<-read.table("3M-february-2018.txt", header=F)
bc<-read.table("scrna_barcodes.tsv", header=F)
table_sel<-table[table$V2%in%bc$V1,]
write.table(table_sel$V1, file="barcodes_perturb_guide.tsv", quote=F, row.names=F, col.names=F)
write.table(table_sel, file="barcodes_perturb_guide_RNA_map.tsv", quote=F, row.names=F, col.names=F)
```
#####
#### 5) Download guiderna_tool from https://github.com/gaofan83/single_cell_perturb_seq/releases/tag/v.1.0.0
`tar -xvzf v.1.0.0.tar.gz`
#####
#### Run the following command line to quantify guide feature counts
```
single_cell_perturb_seq/guiderna/guiderna_tool --R1 SRR13587911_1.fastq --R2 SRR13587911_2.fastq --bc barcodes_perturb_guide.tsv --gd1 guides1.txt --gd2 guides2.txt
```
#####
#### 6) Generate the binarized guide feature count matrices (only `ft_count_cap1_gd1_bin.txt` and `ft_count_cap2_gd2_bin.txt` will be generated) 
`Rscript --vanilla single_cell_perturb_seq/guiderna/binarize_umi_count.R`
#####
##### Note: when integrating guide feature count matrices （with or w/o binarization) with single-cell RNA-seq count matrices, remember to use `barcodes_perturb_guide_RNA_map.tsv` file to convert guide cell barcodes to RNA cell barcode sequences before the integration.
                                     

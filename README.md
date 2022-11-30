[![GitHub Downloads](https://img.shields.io/github/downloads/gaofan83/single_cell_perturb_seq/total.svg?style=social&logo=github&label=Download)](https://github.com/gaofan83/single_cell_perturb_seq/releases)

# single-cell Perturb-Seq pipelines

### This repository includes
* 1) Hashtag data processing pipeline to quantify tag UMIs in individual cells and binarize the count table using Gaussian Mixed Modeling for model-based clustering; 
* 2) Guide RNA data (a novel design of dual guides in vector backbone) processing pipeline to quantify guide UMIs in individual cells and binarize the count table using Gaussian Mixed Modeling; 
* 3) Benchmarking of a test guide RNA dataset generated from 10x Genomics platform. Both in-house guide processing tool and Cellranger3 were used for comparison.

![Workflow](/single_cell_perturb_seq.png)

#### Directory contents 
* [Hashtag pipeline](/hashtag): A folder with RUST executable file.
* [Guide RNA pipeline](/guiderna): A folder with RUST executable file. 
* [Benchmarking of guide RNA data](/benchmarking): A folder with benchmarking results.

#### Installation of `hashtag_tool` and `guiderna_tool`
```
cd ~
wget https://github.com/gaofan83/single_cell_perturb_seq/releases/download/v.1.0.0/v.1.0.0.tar.gz
tar -xvzf v.1.0.0.tar.gz
chmod +x single_cell_perturb_seq/hashtag/hashtag_tool
chmod +x single_cell_perturb_seq/guiderna/guiderna_tool
export PATH=~/single_cell_perturb_seq/hashtag/:$PATH
export PATH=~/single_cell_perturb_seq/guiderna/:$PATH
```

#### Citation
```
Please cite the following paper when using this software:
W Zhou, F Gao, M Romero-Wolf, S Jo, EV Rothenberg. Science Immunology (2022)
"Single-cell deletion analyses show control of pro–T cell developmental speed and pathways by Tcf7, Spi1, Gata3, Bcl11a, Erg, and Bcl11b." 
```

### An online resource to understand the basics of 10X single-cell perturb-seq
[Sarah Teichmann's group at EMBL-EBI / Wellcome Trust Sanger Institute](https://teichlab.github.io/scg_lib_structs/methods_html/10xChromium3fb.html)

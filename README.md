# single-cell Perturb-Seq pipelines

### This repository includes
* 1) Hashtag data processing pipeline to quantify tag UMIs in individual cells and binarize the count table using Gaussian Mixed Modeling for model-based clustering; 
* 2) Guide RNA data (a novel design of dual guides in vector backbone) processing pipeline to quantify guide UMIs in individual cells and binarize the count table using Gaussian Mixed Modeling; 
* 3) Benchmarking of a test guide RNA dataset generated from 10x Genomics platform. Both in-house guide processing tool and Cellranger3 were used for comparison.

![Workflow](/single_cell_perturb_seq.png)

#### Directory contents 
* [Hashtag pipeline](/hashtag): A folder with RUST excutable file.
* [Guide RNA pipeline](/guiderna): A folder with RUST excutable file. 
* [Benchmarking of guide RNA data](/benchmarking): A folder with benchmarking results.

#### Installation of `hashtag_tool` and `guiderna_tool`
```
git clone https://github.com/gaofan83/single_cell_perturb_seq
chmod +x single_cell_perturb_seq/hashtag/hashtag_tool
chmod +x single_cell_perturb_seq/guiderna/guiderna_tool
export PATH=~/single_cell_perturb_seq/hashtag/:$PATH
export PATH=~/single_cell_perturb_seq/guiderna/:$PATH
```

### An online resource to understand the basics of 10X single-cell perturb-seq
[Sarah Teichmann's group at EMBL-EBI / Wellcome Trust Sanger Institute](https://teichlab.github.io/scg_lib_structs/methods_html/10xChromium3fb.html)

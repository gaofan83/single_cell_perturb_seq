# single-cell Perturb-Seq pipelines

### This repository includes
* 1) Hashtag data processing pipeline to quantify tag UMIs in individual cells and binarize the count table using Gaussian Mixed Modeling for model-based clustering; 
* 2) Guide RNA data (a novel design of dual guides in vector backbone) processing pipeline to quantify guide UMIs in individual cells and binarize the count table using Gaussian Mixed Modeling; 
* 3) Benchmarking of a test guide RNA dataset generated from 10x Genomics platform. Both in-house guide processing tool and Cellranger3 were used for comparison.

![Workflow](/single_cell_perturb_seq.png)

#### Directory contents 
* [Hashtag pipeline](/hashtag): A folder with RUST excutable file.
* [Guide RNA pipeline](/guide): A folder with RUST excutable file. 
* [Benchmarking of guide RNA data]/(benchmarking): A folder with benchmarking results.

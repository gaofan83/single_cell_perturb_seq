### Benchmarking of dual-guide single-cell data using **guiderna_tool** and **Cellranger3** pipelines

In dual-guide system, two guide-RNA sites (targeting different sites of the same gene) are engineered in the same vector backbone. In infected cells, dual-guide is expected to perform better than single-guide for gene knock-down. </br>
Based on 10XG bead chemistry, **Capture1** (5'- GCTTTAAGGCCGGTCCTAGCAA -3') and **Capture2** (5'- GCTCACCTATTAGCGGCTAAGG -3') sequences recognize expressed **Guide1** and **Guide2** RNA molecules that have reverse complement capture sequences inserted. Specifically, **Capture1** and **Capture2** sequences should pair with **Guide1** and **Guide2**, respectively.</br>
From single-cell guideRNA data, UMI counts can be calculated for **Guide1** list of barcodes and **Guide2** list of barcodes. As note, **guiderna_tool** uses both capture sequences in R1 reads and template switching oligo sequence (TSO) in R2 read for read filtering and sorting; then potential **protospacer** sequences in R2 reads (after 5' TSO sequence) are mapped against the corresponding guide library (**Guide1** or **Guide2**) for quantification. In contrast, **Cellranger** finds a constant region after **protospacer** region in R2 first, then **protospacer** abundances in R2 are calculated. Since **guiderna_tool** utilizes both R1 and R2 read information for filtering, it is expected to be more accurate. 

Figure 1, Correlation of Guide1 UMI counts from **guiderna_tool** and **Cellranger3**. </br>
![Figure 1](/benchmarking/figure1.png)
####
####
Figure 2, Correlation of Guide2 UMI counts from **guiderna_tool** and **Cellranger3**. </br>
![Figure 2](/benchmarking/figure2.png)
####

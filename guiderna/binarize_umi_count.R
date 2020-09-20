library(MASS)
library(mclust, quietly=TRUE)
library(sBIC)


feature_binary<-function(ft_count){
  ft_count_log<-log10(ft_count + 1)
  ft_count_binary<-ft_count
    for (i in 1:ncol(ft_count)) {
      if(sum(ft_count[,i])>=1){
        fit <- Mclust(ft_count_log[,i], G=2)
        fit$parameters
        if(!is.null(fit)) {ft_count_binary[,i]<-fit$classification
		png(paste("./guide_",colnames(ft_count_binary)[i],".png", sep=""))
		plot(fit, what="density", xlab="log10(UMI+1)")
		rug(ft_count_log[fit$classification == 1, i], col="blue")
		rug(ft_count_log[fit$classification == 2, i], col="red")
		legend("topright", title=colnames(ft_count_binary)[i], c("Negative", "Positive"), fill=c("blue", "red"))
		dev.off()
		} else {ft_count_binary[,i] = rep(0,nrow(ft_count_log))}
      } else {ft_count_binary[,i] = rep(0,nrow(ft_count_log))}
      
    }
    ft_count_binary[ft_count_binary==1] <- 0
    ft_count_binary[ft_count_binary==2] <- 1
    return(ft_count_binary)
}

umi_inhouse1<-read.table("./ft_count_cap1_gd1.txt", header=T, row.names=1)
umi_inhouse2<-read.table("./ft_count_cap2_gd2.txt", header=T, row.names=1)

umi_inhouse1_bin <- feature_binary(umi_inhouse1)
umi_inhouse2_bin <- feature_binary(umi_inhouse2)

write.table(umi_inhouse1_bin, file="./ft_count_cap1_gd1_bin.txt", quote=F, row.names=T, col.names=T)
write.table(umi_inhouse2_bin, file="./ft_count_cap2_gd2_bin.txt", quote=F, row.names=T, col.names=T)



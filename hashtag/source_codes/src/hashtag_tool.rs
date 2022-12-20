extern crate csv;
extern crate needletail;

use std::env;
use needletail::{parse_sequence_path, Sequence};
use std::fs::File;
use std::io::{Read, self, prelude::*, BufReader};
use std::iter::FromIterator;
use std::fs;
use itertools::join;
use std::collections::HashMap;
use std::convert::TryFrom;
use clap::{Arg, App, AppSettings};

fn lines_from_file(filename: &str) -> Vec<String> {
	let mut file = match File::open(filename) {
	Ok(file) => file,
	Err(_) => panic!("no such file"),
	};
	let mut file_contents = String::new();
	file.read_to_string(&mut file_contents).ok().expect("failed to read!");
	let lines: Vec<String> = file_contents.split("\n").map(|s: &str| s.to_string()).collect();
	lines
}


pub fn run() {
	let app_match = App::new("\nSoftware Name: hashtag_tool\n")
		.version("1.0")
		.author("\nAuthor: Fan Gao, Caltech Bioinformatics Resource Center, <gaofan9503@gmail.com>\n")
		.about("\nDescription: This software processes single-cell hashtag data, e.g. antibody-conjugated data from 10X single-cell platform.\n\n\nExample:\nhashtag_tool --R1 R1.fastq --R2 R2.fastq --bc bc.txt --ft hash.txt --bc_len 16 --umi_len 12 --hashtag_len 8 \n")
		.arg(Arg::with_name("Read1") 
			.long("R1")
			.takes_value(true)
			.help("R1 raw fastq file (doesn't work for *gz file)"))
		.arg(Arg::with_name("Read2")
                        .long("R2")
                        .takes_value(true)
                        .help("R2 raw fastq file (doesn't work for *gz file)"))
                .arg(Arg::with_name("barcodes")
                        .long("bc")
                        .takes_value(true)
                        .help("A list of cell barcodes"))
                .arg(Arg::with_name("features")
                        .long("ft")
                        .takes_value(true)
                        .help("A list of hashtag barcodes"))
                .arg(Arg::with_name("barcode_len")
                        .long("bc_len")
                        .takes_value(true)
                        .help("the length of barcodes"))
                .arg(Arg::with_name("umi_len")
                        .long("umi_len")
                        .takes_value(true)
                        .help("the length of UMI"))
                .arg(Arg::with_name("hashtag_len")
                        .long("hashtag_len")
                        .takes_value(true)
                        .help("the length of hashtag"))
		.setting(AppSettings::ArgRequiredElseHelp)
		.get_matches();

   	let fq1_in = app_match.value_of("Read1").unwrap();
	let fq2_in = app_match.value_of("Read2").unwrap();
        let mut nbase: u32 = 0;
	let mut barcode1 = Vec::new();
	let mut feature2 = Vec::new();
	let bc_in = app_match.value_of("barcodes").unwrap();  //bc list from cellranger filtered cells
	let ft_in = app_match.value_of("features").unwrap();  //reference feature sequences
	let bc1 = lines_from_file(&bc_in);
	let ft1 = lines_from_file(&ft_in);
	let len_bc = app_match.value_of("barcode_len").unwrap();
	let len_umi = app_match.value_of("umi_len").unwrap();
	let len_tag = app_match.value_of("hashtag_len").unwrap();
	let bc = len_bc.trim().parse::<usize>().unwrap();
    	let umi = len_umi.trim().parse::<usize>().unwrap();
	let tag = len_tag.trim().parse::<usize>().unwrap();
	let fq1_len = bc + umi;
	let fq2_len = tag;
//	println!("{:?}", bc_in.as_bytes());

//	Create hashmap for reference barcodes and features.
	let mut bc_hashmap: HashMap<String, usize> = HashMap::new();
	let mut ft_hashmap: HashMap<String, usize> = HashMap::new();

	for i in 0..bc1.len() {
	   bc_hashmap.insert(bc1[i].to_string(), i);
	}

        for i in 0..ft1.len() {
           ft_hashmap.insert(ft1[i].to_string(), i);
        }

//	let mut bc_demo = "TTTGTTGTCCTTATAC";
//	let mut ft_demo = "TATGCTGCCACGGTA";
//	match bc_hashmap.get(&bc_demo.to_string()) {
//	    Some(pos) => println!("{}", pos),
//	    None => println!("{} is not found", bc_demo)
//	}

//	Read fq1 file and extract barcode and umi.
        parse_sequence_path(fq1_in, |_| {}, |seq| {
	    let barcode_binary = &seq.seq[0..fq1_len]; //10XG v3 bc+umi
	    let barcode = String::from_utf8((&barcode_binary).to_vec()).unwrap();
	    barcode1.push(barcode);
        }).expect("Error parsing");
	println!("Finish loading read1 fastq file\n");

//	Read fq2 file and extract feature.
	parse_sequence_path(fq2_in, |_| {}, |seq| {
            let feature_binary = &seq.seq[0..fq2_len];
            let feature = String::from_utf8((&feature_binary).to_vec()).unwrap();
            feature2.push(feature);
        }).expect("Error parsing");
	println!("Finish loading read2 fastq file\n");

//	Combine fq1 fq2 sequences.
	let mut combine = Vec::new();
	for i in 0..barcode1.len() {
	    let bc_element = &barcode1[i];
	    let ft_element = &feature2[i];
	    let comb_element = format!("{}{}", bc_element, ft_element);
	    combine.push(comb_element);
	};
	combine.sort();
//	println!("{:?}", combine[0]);

//	Feature map and count.
	let mut bcumi_previous = "";
	let mut ft_previous = "";
	let mut ft_umi_consist ="";
	let mut ftcount = vec![vec![0; ft1.len()-1]; bc1.len()-1];

	for element in &combine {
	    nbase += 1;
	    let bcumi_current = &element[0..fq1_len];
	    let ft_current = &element[fq1_len..(fq1_len + fq2_len)];
	    if bcumi_current.as_bytes() == bcumi_previous.as_bytes() {
		if ft_current.as_bytes() != ft_previous.as_bytes() {
		    ft_umi_consist = "no";
		};
	    }
	    else {let bc_current = &element[0..bc];
		  if (ft_umi_consist == "yes") {
		      match ft_hashmap.get(&ft_current.to_string()) {
			Some(pos_ft) => match bc_hashmap.get(&bc_current.to_string()) {
					  Some(pos_bc) => ftcount[*pos_bc as usize][*pos_ft as usize] += 1,
					  None => (),
					}
		        None => (),
		     }

		  };
		  ft_umi_consist = "yes";
	    };
	    bcumi_previous = bcumi_current;
	    ft_previous = ft_current;
	    if (nbase % 1000000 == 0) {println!("Now {} reads were processed for feature capture.", nbase);};
	};
	let mut output = Vec::new();
	let feature_id = join(&ft1[0..ft1.len()-1], "\t");
	let colnames = format!("CELL_ID\t{}", feature_id);
	output.push(colnames);	
	for i in 0..bc1.len()-1 {
		let ft_count_per_cell = join(&ftcount[i][0..ft1.len()-1], "\t");
		let output_element = format!("{}\t{}", bc1[i], ft_count_per_cell);
		output.push(output_element);
	};
	let output_file = join(&output, "\n");
	fs::write("./ft_count.txt", output_file).expect("Unable to write file");
}

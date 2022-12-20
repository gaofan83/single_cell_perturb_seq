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


fn guides_count(ft: &Vec<String>, bc: &Vec<String>, combine: &Vec<String>, bc_hashmap: &HashMap<String, usize>, ft_hashmap: &HashMap<String, usize>) -> Vec<String> {
	let mut bcumi_previous = "";
	let mut ft_previous = "";
	let mut ft_umi_consist = "";
	let mut ftcount = vec![vec![0; ft.len()-1]; bc.len()-1];
        let mut nbase: u32 = 0;

	for element in combine {
	    nbase += 1;
	    let bcumi_current = &element[0..28];
	    let ft_current = &element[28..52];
	    if bcumi_current.as_bytes() == bcumi_previous.as_bytes() {
		if ft_current.as_bytes() != ft_previous.as_bytes() {
		    ft_umi_consist = "no";
		};
	    }
	    else {let bc_current = &element[0..16];
		  if (ft_umi_consist == "yes") {
		      let ft_match1 = &ft_current[0..20];
		      match ft_hashmap.get(&ft_match1.to_string()) {
			Some(pos_ft) => match bc_hashmap.get(&bc_current.to_string()) {
					  Some(pos_bc) => ftcount[*pos_bc as usize][*pos_ft as usize] += 1,
					  None => (),
					}
		        None => (),
		     }

		      let ft_match2 = &ft_current[1..21];
                      match ft_hashmap.get(&ft_match2.to_string()) {
                        Some(pos_ft) => match bc_hashmap.get(&bc_current.to_string()) {
                                          Some(pos_bc) => ftcount[*pos_bc as usize][*pos_ft as usize] += 1,
                                          None => (),
                                        }
                        None => (),
                     }

                      let ft_match3 = &ft_current[2..22];
                      match ft_hashmap.get(&ft_match3.to_string()) {
                        Some(pos_ft) => match bc_hashmap.get(&bc_current.to_string()) {
                                          Some(pos_bc) => ftcount[*pos_bc as usize][*pos_ft as usize] += 1,
                                          None => (),
                                        }
                        None => (),
                     }

                      let ft_match4 = &ft_current[3..23];
                      match ft_hashmap.get(&ft_match4.to_string()) {
                        Some(pos_ft) => match bc_hashmap.get(&bc_current.to_string()) {
                                          Some(pos_bc) => ftcount[*pos_bc as usize][*pos_ft as usize] += 1,
                                          None => (),
                                        }
                        None => (),
                     }

                      let ft_match5 = &ft_current[4..24];
                      match ft_hashmap.get(&ft_match5.to_string()) {
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
//	    if (nbase % 1000000 == 0) {println!("Now {} reads were processed for feature capture.", nbase);};
	};
	let mut output = Vec::new();
	let feature_id = join(&ft[0..ft.len()-1], "\t");
	let colnames = format!("CELL_ID\t{}", feature_id);
	output.push(colnames);	
	for i in 0..bc.len()-1 {
		let ft_count_per_cell = join(&ftcount[i][0..ft.len()-1], "\t");
		let output_element = format!("{}\t{}", bc[i], ft_count_per_cell);
		output.push(output_element);
	};
	output
}

pub fn run() {
	let app_match = App::new("\nSoftware Name: guiderna_tool\n")
		.version("1.0")
		.author("\nAuthor: Fan Gao, Caltech Bioinformatics Resource Center, <gaofan9503@gmail.com>\n")
		.about("\nDescription: This software processes single-cell dual-guide RNA data generated from 10XG V3 chemistry.\n\n\nExample:\nguiderna_tool --R1 R1.fastq --R2 R2.fastq --bc barcodes.txt --gd1 guides1.txt --gd2 guides2.txt\n")
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
                .arg(Arg::with_name("guides1")
                        .long("gd1")
                        .takes_value(true)
                        .help("List1 of sgRNA protospacer sequences that should be captured by Capture1 sequence"))
                .arg(Arg::with_name("guides2")
                        .long("gd2")
                        .takes_value(true)
                        .help("List2 of sgRNA protospacer sequences that should be captured by Capture2 sequence"))
		.setting(AppSettings::ArgRequiredElseHelp)
		.get_matches();

   	let fq1_in = app_match.value_of("Read1").unwrap();
	let fq2_in = app_match.value_of("Read2").unwrap();
	let mut barcode1 = Vec::new();
	let mut feature2 = Vec::new();
	let bc_in = app_match.value_of("barcodes").unwrap();  // bc list from cellranger filtered cells (converted to the corresponding feature barcodes
	let gd1_in = app_match.value_of("guides1").unwrap();  // sgRNA reference1 sequences
	let gd2_in = app_match.value_of("guides2").unwrap();  // sgRNA reference2 sequences
	let bc1 = lines_from_file(&bc_in);
	let gd1 = lines_from_file(&gd1_in);
	let gd2 = lines_from_file(&gd2_in);

//	Create hashmap for reference barcodes and sgRNAs.
	let mut bc1_hashmap: HashMap<String, usize> = HashMap::new();
	let mut gd1_hashmap: HashMap<String, usize> = HashMap::new();
	let mut gd2_hashmap: HashMap<String, usize> = HashMap::new();

	for i in 0..bc1.len() {
	   bc1_hashmap.insert(bc1[i].to_string(), i);
	}

        for i in 0..gd1.len() {
           gd1_hashmap.insert(gd1[i].to_string(), i);
        }

        for i in 0..gd2.len() {
           gd2_hashmap.insert(gd2[i].to_string(), i);
        }

//	Read fq1 file and extract 16-bp barcode, 12-bp umi and 22-bp capture sequence.
        parse_sequence_path(fq1_in, |_| {}, |seq| {
	    let barcode_binary = &seq.seq[0..50]; //10XG v3 bc+umi+capture
	    let barcode = String::from_utf8((&barcode_binary).to_vec()).unwrap();
	    barcode1.push(barcode);
        }).expect("Error parsing");
	println!("Finish loading read1 fastq file\n");

//	Read fq2 file and extract 30-bp capture and 22-bp potential sgRNA feature (consider 2bp shift of 20bp sgRNA).
	parse_sequence_path(fq2_in, |_| {}, |seq| {
            let feature_binary = &seq.seq[0..52];
            let feature = String::from_utf8((&feature_binary).to_vec()).unwrap();
            feature2.push(feature);
        }).expect("Error parsing");
	println!("Finish loading read2 fastq file\n");

//	Capture sequences to filter raw reads (22-bp from R1 and 28-bp from R2)
	let cap1 = "TTGCTAGGACCGGCCTTAAAGCAAGCAGTGGTATCAACGCAGAGTACATG";
	let cap2 = "CCTTAGCCGCTAATAGGTGAGCAAGCAGTGGTATCAACGCAGAGTACATG";

//	Filter reads using cap1 & cap2 sequences, and generate fq1&fq2 concat records for sgRNA1 & sgRNA2 separately.
	let mut combine1 = Vec::new();
	let mut combine2 = Vec::new();
	for i in 0..barcode1.len() {
	    let bc_element = &barcode1[i];
	    let ft_element = &feature2[i];
	    let cap_element = format!("{}{}", &bc_element[28..50], &ft_element[0..28]);
	    if(cap_element == cap1){
		let comb_element1 = format!("{}{}", &bc_element[0..28], &ft_element[28..52]);
		combine1.push(comb_element1);}
	    if(cap_element == cap2){
                let comb_element2 = format!("{}{}", &bc_element[0..28], &ft_element[28..52]);
                combine2.push(comb_element2);}
	};
	combine1.sort();
	combine2.sort();

//	Feature map and count.
	println!("Count cap1 & guide1 combinatin");
	let output1 = guides_count(&gd1, &bc1, &combine1, &bc1_hashmap, &gd1_hashmap);
        let output1_file = join(&output1, "\n");
        fs::write("./ft_count_cap1_gd1.txt", output1_file).expect("Unable to write file");

	println!("Count cap2 & guide2 combinatin");
	let output2 = guides_count(&gd2, &bc1, &combine2, &bc1_hashmap, &gd2_hashmap);
        let output2_file = join(&output2, "\n");
        fs::write("./ft_count_cap2_gd2.txt", output2_file).expect("Unable to write file");

	println!("Count cap2 & guide1 combinatin");
        let output3 = guides_count(&gd1, &bc1, &combine2, &bc1_hashmap, &gd1_hashmap);
        let output3_file = join(&output3, "\n");
        fs::write("./ft_count_cap2_gd1.txt", output3_file).expect("Unable to write file");

	println!("Count cap1 & guide2 combinatin");
        let output4 = guides_count(&gd2, &bc1, &combine1, &bc1_hashmap, &gd2_hashmap);
        let output4_file = join(&output4, "\n");
        fs::write("./ft_count_cap1_gd2.txt", output4_file).expect("Unable to write file");

}

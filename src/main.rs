extern crate nom;
use sap_time_extractor::{BoxError,parsers, TimeType};
use std::{fs::read_to_string,};





fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn main() -> std::result::Result<(), BoxError>{
	let lines = read_lines("/Users/kareldewitte/Documents/Projects/TimeExtractor/sap_time_extractor/resources/TimeStatement_20230401_20230813.txt");
    let mut time_office = 0.0;
    let mut time_travel = 0.0;
    let mut time_home = 0.0;
    for line in lines{
        match  parsers::parse_timeline(line.as_str()) {
            Ok(res)=>{
                let timeline = res.1;
                if timeline.timeType == TimeType::ONSITE || timeline.timeType == TimeType::REM {
                    time_office = time_office + timeline.time;
                } else
                if timeline.timeType == TimeType::WFH  {
                    time_home = time_home + timeline.time;
                } else
                if timeline.timeType == TimeType::TRAVEL {
                    time_travel = time_travel + timeline.time;
                }

            },
            Err(_)=>{

            }
            
        };
    }
    let sum = time_home + time_office;
    println!("wfh:{}, onsite:{}, travel:{}",time_home/sum,time_office/sum,time_travel);  

    Ok(())
}
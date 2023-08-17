mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/// Read one byte from the file at a given offset.
#[wasm_bindgen]
pub fn read_file(file: web_sys::File) -> f64 {
    let mut wf = file;
    wf.size()

    // let content = pdf_extract::extract_text("resources/TimeStatement_20230401_20230817.pdf").unwrap();
    
    // // for line in content.lines(){
    // //     println!("{}",line);
    // // }

	// let lines = content.lines();//read_lines("resources/TimeStatement_20230401_20230813.txt");
    // let mut time_office = 0.0;
    // let mut time_travel = 0.0;
    // let mut time_home = 0.0;
    // for line in lines{
    //     match  parsers::parse_timeline(line) {
    //         Ok(res)=>{
    //             let timeline = res.1;
    //             if timeline.timeType == TimeType::ONSITE || timeline.timeType == TimeType::REM {
    //                 time_office = time_office + timeline.time;
    //             } else
    //             if timeline.timeType == TimeType::WFH  {
    //                 time_home = time_home + timeline.time;
    //             } else
    //             if timeline.timeType == TimeType::TRAVEL {
    //                 time_travel = time_travel + timeline.time;
    //             }

    //         },
    //         Err(_)=>{

    //         }
            
    //     };
    // }
    // let sum = time_home + time_office+time_travel;
    // println!("wfh:{}h - {}%, onsite:{}h - {}%, travel:{}, total work:{}",time_home,time_home/sum,time_office,time_office/sum,time_travel/sum,sum);  

    // Ok(())



}







#[derive(Clone, Default, Debug)]
pub struct TimeLine {
	pub time: f32,
	pub timeType: TimeType,
}



#[derive(PartialEq)]
#[derive(Clone, Default, Debug)]
pub enum TimeType {
    WFH,
    REM,
    #[default]
    ONSITE,
    TRAVEL,
    OFF
}

impl From<&str> for TimeType {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "business absence" => TimeType::TRAVEL,
            "working from home" => TimeType::WFH,
            "terminal-id" => TimeType::ONSITE,
            "working remotely" => TimeType::REM,
            "employee not present" => TimeType::OFF,
            _ => unimplemented!("no other types supported"),
        }
    }
}


pub type BoxError = std::boxed::Box<dyn
	std::error::Error   // must implement Error to satisfy ?
	+ std::marker::Send // needed for threads
	+ std::marker::Sync // needed for threads
>;


pub mod parsers {
	use std::any;

use nom::{error::context, branch::{alt}, bytes::complete::{tag_no_case, tag}, sequence::{tuple, separated_pair, pair}, combinator::opt, character::complete::{alpha1, digit1, multispace0, space0, space1, digit0, anychar}, complete::take, multi::count};
    use crate::TimeLine;
    use time::Time;
    use super::TimeType;

	fn not_relevant(i: &str) -> nom::IResult<&str, &str> {
		nom::bytes::complete::is_not(" \t|")(i)
	}

    fn terminal_number(input: &str) -> nom::IResult<&str, &str> {
        context(
            "terminal_number",
            alt((tag(" "),digit1),
        ))(input)
        .map(|(next_input,res)| (next_input,res.into()))
    }

    fn terminal_slot(input: &str) -> nom::IResult<&str, &str> {
        context(
            "terminal_slot",
            tuple((
                terminal_number,
                alt((tag(" "),tag(""))),
                tag("/"),
                alt((tag(" "),tag(""))),
                terminal_number ),)
                ,
        )(input)
        .map(|(next_input,res)| (next_input,""))

    }

    fn terminal_id(input: &str) -> nom::IResult<&str, &str> {
        context(
            "terminal_id",
            tuple(
                (tag_no_case("Terminal-ID"),space0,tag(": "),terminal_slot),),
        )(input)
        .map(|(next_input,res)| (next_input,res.0))

    }
    
    fn timetype(input: &str) -> nom::IResult<&str, TimeType> {
        context(
            "timetype",
            alt((tag_no_case("Business absence"), tag_no_case("Working from home"),
                    terminal_id,
                    tag_no_case("Working remotely"),tag_no_case("Employee not present"))),
        )(input)
        .map(|(next_input, res)| (next_input, res.into()))
    }

    fn time(input: &str) -> nom::IResult<&str, Time> {
        context(
            "time",
            tuple((digit1,tag(":"),digit1)),) (input)
            .map(|(next_input, res)|{ 
                    let (hh, sign, mm) = res;
                        (
                            next_input,
                            Time::from_hms(hh.to_string().parse().unwrap(), mm.to_string().parse().unwrap(), 0).unwrap(),
                        )
                    }
                )
    }

    fn number_from_comma(input: &str) -> nom::IResult<&str, f32> {
        context(
            "number_from_comma",
            separated_pair(digit1,tag(","),digit1),)(input)
            .map(|(next_input, res)|{ 
                    let (i, f) = res;
                        (
                            next_input,
                            (i.to_string()+"."+f).parse().unwrap(),
                        )
                    }
                )
    }


    pub fn timeline(input: &str) -> nom::IResult<&str, TimeLine> {
        context(
            "timeLine",
                tuple((digit1, space0, alpha1, space0, timetype, space0, time, space0, time, space0, number_from_comma, )),
             )(input)
        .map(|(next_input, res)| {
            //let (scheme, authority, host, port, path, query, fragment) = res;
            println!("=>{:?}",res);
            let time = res.10;
            let ttype = res.4;
            (
                next_input,
                TimeLine {
                    time:time,
                    timeType:ttype
                },
            )
        })
    }

    pub fn timeline_alt(input: &str) -> nom::IResult<&str, TimeLine> {
        context(
            "timeLineAlt",
                tuple((space0, timetype, space0, time, space0, time, space0, number_from_comma, )),
             )(input)
        .map(|(next_input, res)| {
            //let (scheme, authority, host, port, path, query, fragment) = res;
            println!("=>{:?}",res);
            let time = res.7;
            let ttype = res.1;
            (
                next_input,
                TimeLine {
                    time:time,
                    timeType:ttype
                },
            )
        })
    }

    pub fn parse_timeline(input: &str) -> nom::IResult<&str, TimeLine> {
    
        alt((timeline,timeline_alt))(input)
    
    }
	


	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn test_not_relevant() {
			assert_eq!(not_relevant("abcd efg"), Ok((" efg", "abcd")));
			assert_eq!(not_relevant("abcd\tefg"), Ok(("\tefg", "abcd")));
			//assert_eq!(not_whitespace(" abcdefg"), Err(nom::Err::Error((" abcdefg", nom::error::ErrorKind::IsNot))));
		}


        #[test]
		fn test_time() {
			assert_eq!(time("08:56"), Ok(("", Time::from_hms(8, 56, 0).unwrap())));
			//assert_eq!(not_relevant("abcd\tefg"), Ok(("\tefg", "abcd")));
			//assert_eq!(not_whitespace(" abcdefg"), Err(nom::Err::Error((" abcdefg", nom::error::ErrorKind::IsNot))));
		}

        #[test]
		fn test_number_from_comma() {
			assert_eq!(number_from_comma("7,50"), Ok(("", 7.50)));
			//assert_eq!(not_relevant("abcd\tefg"), Ok(("\tefg", "abcd")));
			//assert_eq!(not_whitespace(" abcdefg"), Err(nom::Err::Error((" abcdefg", nom::error::ErrorKind::IsNot))));
		}

        #[test]
        fn test_terminal_number(){
            
            let _line ="2012 / 2012 07:53   16:40       8,78       8,28       0,28 TZR";
            let _line0 ="2015 /  08:31   17:38      9,11      8,61     0,61 TZR";
            let _line1 =" / 2009  08:00  16:17  8,30   7,80   0,20- TZR";
            let _line2 ="Terminal-ID : 2015 /  08:31   17:38      9,11      8,61     0,61 TZR";
            assert_eq!(terminal_number(_line1),Ok(("/ 2009  08:00  16:17  8,30   7,80   0,20- TZR", " ")));
            assert_eq!(terminal_number(_line0),Ok((" /  08:31   17:38      9,11      8,61     0,61 TZR", "2015")));
            
        }

        #[test]
        fn test_terminal_id(){
            
            let _line ="Terminal-ID : 2012 / 2012 07:53   16:40       8,78       8,28       0,28 TZR";
            let _line0 ="Terminal-ID : 2015 /  08:31   17:38      9,11      8,61     0,61 TZR";
            let _line1 ="Terminal-ID :  / 2009  08:00  16:17  8,30   7,80   0,20- TZR";
            let _line2 ="Terminal-ID : 2015 /  08:31   17:38      9,11      8,61     0,61 TZR";
            
            assert_eq!(terminal_id(_line),Ok((" 07:53   16:40       8,78       8,28       0,28 TZR", "Terminal-ID")));

        }

        #[test]
        fn test_line(){
            let _line = "05 WE    Working remotely                                08:00         13:00         5,00           7,50        0,50- TZR";
            let _line0 = "        Working remotely                                08:00         13:00         5,00           7,50        0,50- TZR";
            
            println!("{:?}",timeline(_line).unwrap());
            println!("{:?}",timeline_alt(_line0).unwrap());
            println!("{:?}",parse_timeline(_line0).unwrap());
            
            //assert_eq!(timetype(_line), Ok(("yay", TimeType::REM)));

        }

	}
}
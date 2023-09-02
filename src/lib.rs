mod utils;

use js_sys::ArrayBuffer;
use time::Time;
use wasm_bindgen::prelude::*;
use lopdf::Document;
use web_sys::Blob;
use serde::{Deserialize, Serialize};
use serde_json::Result;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/// Read one byte from the file at a given offset.
#[wasm_bindgen]
pub fn read_file(le:usize, data: &[u8]) -> String {

    //let mut buffer: Vec<u8> = data.iter().take(le).map(|&x| x as u8).collect();
    let content = extract_text(data);    
    // for line in content.lines(){
    //     println!("{}",line);
    // }
    let res = extract_time_data(parsers::parse_timeline__many0(content.as_str()).unwrap().1);
    //buffer.len()
    serde_json::to_string(&res).unwrap()


}

pub fn extract_time_data(times:Vec<TimeLine>)->TimeResume{
    let mut time_office = 0.0;
    let mut time_travel = 0.0;
    let mut time_home = 0.0;
    for timeline in times.clone(){
        if timeline.timeType == TimeType::ONSITE || timeline.timeType == TimeType::REM {
            time_office = time_office + timeline.time;
        } else
        if timeline.timeType == TimeType::WFH  {
            time_home = time_home + timeline.time;
        } else
        if timeline.timeType == TimeType::TRAVEL {
            time_travel = time_travel + timeline.time;
        }
    }
    let time_sum = time_home + time_office+time_travel;
    //format!("wfh:{}h - {}%, onsite:{}h - {}%, travel:{}, total work:{}",time_home,time_home/sum,time_office,time_office/sum,time_travel/sum,sum)  
    let ratio_wfh = time_home/time_sum;
    let ratio_onsite = time_office/time_sum;
    
    TimeResume{
        time_office,time_travel,time_home,time_sum, ratio_wfh,ratio_onsite,
        times
    }
}

fn extract_text(data: &[u8]) -> String{

    let res = Document::load_mem(data);
    let doc = res.unwrap();
    let pages = doc.get_pages();
    let mut texts = Vec::new();

    for (i, _) in pages.iter().enumerate() {
        let page_number = (i + 1) as u32;
        let text = doc.extract_text(&[page_number]);
        texts.push(text.unwrap_or_default());
    }

    texts.join("")

}


#[derive(Serialize, Deserialize)]
#[derive(Clone, Default, Debug)]
pub struct TimeResume {
	pub time_office: f32,
    pub time_travel: f32,
	pub time_home: f32,
    pub time_sum: f32,

    pub ratio_wfh: f32,
    pub ratio_onsite: f32,  
    
    pub times: Vec<TimeLine>
}



#[derive(Serialize, Deserialize)]
#[derive(Clone, Default, Debug)]
pub struct TimeLine {
    pub day: String,
	pub time: f32,
    pub time_pause_deducted: f32,
	pub timeType: TimeType,
}


#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
#[derive(Clone, Default, Debug)]
pub enum TimeType {
    WFH,
    REM,
    #[default]
    ONSITE,
    TRAVEL,
    OFF,
    NA
}

impl From<&str> for TimeType {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "business absence" => TimeType::TRAVEL,
            "working from home" => TimeType::WFH,
            "terminal-id" => TimeType::ONSITE,
            "working remotely" => TimeType::REM,
            "employee not present" => TimeType::OFF,
            "not applicable" => TimeType::NA,
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
    use nom::{error::context, branch::{alt}, bytes::complete::{tag_no_case, tag, take_until, take_while1}, sequence::{tuple, separated_pair, pair, preceded}, combinator::opt, character::complete::{alpha1, digit1, multispace0, space0, space1, digit0, anychar, newline, not_line_ending}, complete::take, multi::{count, many0, many1}, Parser};
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
    
    fn terminal_id_alt(input: &str) -> nom::IResult<&str, &str> {
        context(
            "terminal_id",
            tuple(
                (tag_no_case("Terminal-ID"),not_line_ending),),
        )(input)
        .map(|(next_input,res)| (next_input,res.0))

    }
    
    fn timetype(input: &str) -> nom::IResult<&str, TimeType> {
        context(
            "timetype",
            alt((tag_no_case("Business absence"), tag_no_case("Working from home"),
                    terminal_id_alt,
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

    fn time_space_new(input: &str) -> nom::IResult<&str, Time> {
        context(
            "time_space_new",
            tuple((space0,time,newline,)),) (input)
            .map(|(next_input, res)|{ 
                    let (sp, time, nl) = res;
                        (
                            next_input,
                            time,
                        )
                    }
                )
    }
    fn nbr_comma_new(input: &str) -> nom::IResult<&str, f32> {
        context(
            "nbr_comma_new",
            tuple((space0, number_from_comma, anychar, newline,)),) (input)
            .map(|(next_input, res)|{ 
                    
                        (
                            next_input,
                            res.1,
                        )
                    }
                )
    }


    pub fn timeline0(input: &str) -> nom::IResult<&str, TimeLine> {
        context(
            "timeLine0",
                tuple((digit1, newline,
                         alpha1, newline,
                         timetype, newline,
                         time_space_new,
                         time_space_new,
                         nbr_comma_new,
                         nbr_comma_new,
                         nbr_comma_new,
                         alt((tag("TZR \n"),tag("TZR A\n"),tag("TZR B\n"))),
                          )),
             )(input)
        .map(|(next_input, res)| {
           

            let time = res.8;
            let timeded = res.9;
            let ttype = res.4;
            let day = res.0.to_owned()+res.2;
            (
                next_input,
                TimeLine {
                    day: day,
                    time:time,
                    time_pause_deducted: timeded,
                    timeType:ttype
                },
            )
        })
    }
    
    pub fn timeline00(input: &str) -> nom::IResult<&str, TimeLine> {
        context(
            "timeLine00",
                tuple((digit1, newline,
                         alpha1, newline,
                         timetype, newline,
                         nbr_comma_new,
                         nbr_comma_new,
                         alt((tag("TZR \n"),tag("TZR A\n"),tag("TZR B\n"))),
                          )),
             )(input)
        .map(|(next_input, res)| {
      
            let time = res.6;
            let timeded = res.6;
            let ttype = res.4;
            let day = res.0.to_owned()+res.2;
            (
                next_input,
                TimeLine {
                    day: day,
                    time:time,
                    time_pause_deducted: timeded,
                    timeType:ttype
                },
            )
        })
    }

    pub fn timeline_absence(input: &str) -> nom::IResult<&str, TimeLine> {
        context(
            "timeLine_absence",
                tuple((digit1, newline,
                         alpha1, newline,
                         timetype, newline,
                         tag("FREI "), newline,
                          )),
             )(input)
        .map(|(next_input, res)| {
            let time = 0.0;
            let ttype = res.4;
            let day = res.0.to_owned()+res.2;
            (
                next_input,
                TimeLine {
                    day: day,
                    time:time,
                    time_pause_deducted: 0.0,
                    timeType:ttype
                },
            )
        })
    }

    pub fn timeline_absence_0(input: &str) -> nom::IResult<&str, TimeLine> {
        context(
            "timeLine_absence_0",
                tuple((digit1, newline,
                         alpha1, newline,
                         not_line_ending, newline,
                         alt((tag("FREI \n"),tag("TZR \n"),tag("TZR A\n"),tag("TZR B\n"))),
                          )),
             )(input)
        .map(|(next_input, res)| {
            let time = 0.0;
            let ttype = res.4;
            let day = res.0.to_owned()+res.2;
            (
                next_input,
                TimeLine {
                    day:day,
                    time:time,
                    time_pause_deducted: 0.0,
                    timeType:TimeType::OFF
                },
            )
        })
    }

    pub fn timeline_absence_1(input: &str) -> nom::IResult<&str, TimeLine> {
        context(
            "timeLine_absence_1",
                tuple((digit1, newline,
                         alpha1, newline,
                         not_line_ending, newline,
                         not_line_ending,newline,
                         not_line_ending,newline,
                         alt((tag("FREI \n"),tag("TZR \n"),tag("TZR A\n"),tag("TZR B\n")))
                          )),
             )(input)
        .map(|(next_input, res)| {
           
            let time = 0.0;
            let ttype = res.4;
            let day = res.0.to_owned()+res.2;
            (
                next_input,
                TimeLine {
                    day:day,
                    time:time,
                    time_pause_deducted: 0.0,
                    timeType:TimeType::OFF
                },
            )
        })
    }


    pub fn timeline0_alt(input: &str) -> nom::IResult<&str, TimeLine> {
        context(
            "timeLine_alt",
                tuple((timetype, newline,
                         time_space_new,
                         time_space_new,
                         nbr_comma_new,
                          )),
             )(input)
        .map(|(next_input, res)| {
          

            let time = res.4;
            //let timeded = res.5;
            let ttype = res.0;
            (
                next_input,
                TimeLine {
                    day: "cont'd".to_string(),
                    time:time,
                    time_pause_deducted: 0.0,
                    timeType:ttype
                },
            )
        })
    }
    


    pub fn timeline(input: &str) -> nom::IResult<&str, TimeLine> {
        context(
            "timeLine",
                tuple((digit1, space0, alpha1, space0, timetype, space0, time, space0, time, space0, number_from_comma, )),
             )(input)
        .map(|(next_input, res)| {
            

            println!("=>{:?}",res);
            let time = res.10;
            let ttype = res.4;
            let day = res.0.to_owned()+res.2;
            (
                next_input,
                TimeLine {
                    day:day,
                    time:time,
                    time_pause_deducted: 0.0,
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
           

            println!("=>{:?}",res);
            let time = res.7;
            let ttype = res.1;
            
            (
                next_input,
                TimeLine {
                    day:"cont'd".to_string(),
                    time:time,
                    time_pause_deducted: 0.0,
                    timeType:ttype
                },
            )
        })
    }

    fn parse_untill_dws(input: &str) -> nom::IResult<&str, &str>{
        context(
            "untill_dws",
                take_until("DWS"),
             )(input)
        .map(|(next_input, res)| 
        {(next_input,res)}
        )

    }
    pub fn parse_weekly(input:&str) -> nom::IResult<&str,TimeLine>{

            tuple((
                tag("Weekly Total :"),newline,
                not_line_ending,newline,
                not_line_ending,newline,
                not_line_ending,newline,
                not_line_ending,newline))(input)
            .map(|(next,res)|{(next,
                    TimeLine{
                        day:"na".to_string(),
                        time:0.0,
                        time_pause_deducted: 0.0,
                        timeType:TimeType::NA
                    }
            )})

    }


    pub fn parse_timeline_single(input: &str) -> nom::IResult<&str, Vec<TimeLine>> {
        context(
            "parse_timeline",
            tuple((
                    parse_untill_dws,
                    tag("DWS"),newline,
                        many0(
                            alt((timeline0,timeline0_alt,timeline00,timeline_absence,timeline_absence_0,timeline_absence_1,parse_weekly))
                        ))
                    ),
                )(input).map(|(g,r)|
                {
                    (
                        g,
                        r.3
                    )

                }
                )
            
    }


    pub fn parse_timeline__many0(input: &str)-> Result<(&str, Vec<TimeLine>), nom::Err<nom::error::Error<&str>>> {
    
        //alt((pair(timeline0,timeline0_alt).map(|g|g.1),timeline0))(input)
         many0(parse_timeline_single)(input).map(|(g,r)|
                {
                    let flat = r.into_iter().flatten().collect::<Vec<TimeLine>>();
                    (g,flat)             
                }
            )
        
    }


   

    pub fn parse_timeline(input: &str) -> nom::IResult<&str, TimeLine> {
    
        alt((timeline,timeline_alt))(input)
    
    }
	


	#[cfg(test)]
	mod tests {
		use std::fs;

use crate::extract_time_data;

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
            let _line3 ="Terminal-ID : 2009 / \n 07:51\n 17:08\n 9,28 \n 8,78 \n 0,78 \nTZR \n04\nFR\nWorking from home\n";
            assert_eq!(terminal_id_alt(_line3),Ok((" 07:53   16:40       8,78       8,28       0,28 TZR", "Terminal-ID")));

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

        #[test]
        fn test_line0(){
            let _line = "Your time data could only be correctly evaluated up to and including 16.08.2023.        Working from home\n 08:00\n 14:00\n  6,00 \n 5,75 \n 2,25-\nTZR \n";
            let _line0 = "Business absence\n 14:00\n 18:00\n 4,00 \n";
            let _line1 = _line.to_owned()+_line0;
            let _line2 = "06\nTH\nWorking from home\n 08:00\n 16:00\n  8,00 \n 7,50 \n 3,50 \nTZR B\n06\nTH\n1/2 Day Off\nTZR B\n07\nFR\nGood Friday\nTZR A\n";
            
            //println!("{:?}",timeline0(_line2));
            let line = fs::read_to_string("src/test/raw-text.txt").unwrap();
            //println!("{:?}",parse_timeline_single(line.as_str()));
            //println!("{:?}",timeline0_alt(_line0).unwrap());
            let results = parse_timeline__many0(line.as_str()).unwrap().1;
            extract_time_data(results);
            //println!("results {:?}",results.1);
            //assert_eq!(timetype(_line), Ok(("yay", TimeType::REM)));

        }

	}
}
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
	use nom::{error::context, branch::{alt}, bytes::complete::{tag_no_case, tag}, sequence::{tuple, separated_pair}, combinator::opt, character::complete::{alpha1, digit1, multispace0, space0}};
    use crate::TimeLine;
    use time::Time;
    use super::TimeType;

	fn not_relevant(i: &str) -> nom::IResult<&str, &str> {
		nom::bytes::complete::is_not(" \t|")(i)
	}

    fn terminal_id(input: &str) -> nom::IResult<&str, &str> {
        context(
            "terminal_id",
            tuple(
                (tag_no_case("Terminal-ID"),space0,tag(":"),space0,opt(digit1),opt(space0),tag("/"),space0,alt((space0,digit1))),),
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
        fn test_terminal_id(){
            
            let _line ="Terminal-ID : 2012 /                                               07:53   16:40       8,78       8,28       0,28 TZR";
            let _line0 ="Terminal-ID : 2015 /                     08:31   17:38      9,11      8,61     0,61 TZR";
            println!("{:?}",terminal_id(_line).unwrap());
            println!("{:?}",terminal_id(_line0).unwrap());
            
            //assert_eq!(timetype(_line), Ok(("yay", TimeType::REM)));

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
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::fs;
use chrono::{DateTime, FixedOffset, ParseError};
use crossbeam_skiplist::{SkipMap};
use lazy_static::lazy_static;
use regex::Regex;
use walkdir::WalkDir;
use dirs::home_dir;
use tracing::{info, warn, trace};
use rayon::prelude::*;
use encoding::{Encoding, DecoderTrap, all::UTF_8};

lazy_static! {
    pub static ref LOGFILE_RX: Regex = Regex::new(r"(?m)^(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}(?:-|\+)\d{2}:\d{2}) (\d{1,2}),([^,]+),(.*)$").expect("Can't parse logfile regex!");
    pub static ref DEFAULT_LOG_DIR: PathBuf = [home_dir().expect("Can't find your home directory!").to_str().unwrap(), "Documents", "Elder Scrolls Online", "live", "Logs"].iter().collect();
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Logentry {
    pub stamp: DateTime<FixedOffset>,
    pub channel_id: i32,
    pub speaker: String,
    pub message: String
}

pub type Elt = (DateTime<FixedOffset>, Logentry);

pub fn parse(input: &str) -> Result<SkipMap<DateTime<FixedOffset>, Logentry>, ParseError> {
    trace!("Starting parse for {}", input);
    let retval = SkipMap::new();
    LOGFILE_RX.captures_iter(input).par_bridge().for_each(|cap| {
        trace!("Match: {}", &cap[0]);
        let dt = DateTime::parse_from_str(&cap[1], "%Y-%m-%dT%H:%M:%S%.3f%:z");
        match dt {
            Ok(pdt) => {
                retval.insert(pdt, Logentry {
                    stamp: pdt,
                    channel_id: cap[2].parse::<i32>().unwrap(),
                    speaker: cap[3].to_string(),
                    message: cap[4].trim().to_string(),
                });
            }
            Err(e) => trace!("{}", e),
        }
    }); 
        
    return Result::Ok(retval);
}

pub fn parse_all(a: &mut dyn Iterator<Item = PathBuf>) -> Result<SkipMap<DateTime<FixedOffset>, Logentry>, ParseError> {  
    let mut buf = String::from("");
    for fil in a.into_iter() {
        let m = fs::read(&fil);
        match m {
            Ok(okm) => {
                info!("Appending contents of {:?}", &fil);
                match UTF_8.decode(okm.as_slice(), DecoderTrap::Replace) {
                    Ok(okmm) => {
                        buf += &okmm;
                    },
                    Err(_) => ()
                }
                
            },
            Err(errm) => info!("For file '{:?}' error reading: {}", &fil, errm),
        }
        buf += "\n"; // Ensure the next file starts on a new line. Extra newline will be ignored by the regex matcher in parse().
    }
    return parse(buf.as_str());
}

pub fn find_logfiles(path: &str) -> BTreeSet<PathBuf> {
    let mut retval = BTreeSet::new();
    for dir in WalkDir::new(path).follow_links(true).into_iter() {
        match dir {
            Ok(ud) => {
                let p = ud.path();
                if !p.is_dir() {
                    lazy_static! {
                        static ref VALID_EXTS: Vec<String> =
                            vec![String::from("log"), String::from("txt")];
                    }
                    if let Some(ext) = p.extension() {
                        if VALID_EXTS.contains(&ext.to_string_lossy().to_string()) {
                            retval.insert(p.to_path_buf());
                        }
                        else {
                            info!("Skipping file '{:?}' because I don't think it's a .log or .txt file", p);
                        }
                    }
                    else {
                        info!("Skipping file '{:?}' because I don't know its extension", p);
                    }
                }
            }
            Err(e) => {
                warn!("Bad directory or filename {}", e.to_string());
            }
        }
    }
    trace!("File list: {:?}", retval);
    return retval;
}


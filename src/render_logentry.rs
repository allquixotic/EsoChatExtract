use crate::parse_logfile::Logentry;
use chrono::{Timelike, NaiveDate};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEF_PALETTE: Vec<String> = vec!["002b36", "073642", "586e75", "657b83", "839496", "93a1a1", "eee8d5", "fdf6e3", "b58900", "cb4b16", "dc322f", "d33682", "6c71c4", "268bd2", "2aa198", "859900"].iter().map(|&x| x.to_string()).collect();
}

pub fn render_plain(itr: impl Iterator<Item=Logentry>) -> String {
    let mut buf = String::from("----RENDER PLAIN----");
    let mut curr_date: Option<NaiveDate> = None;
    for entry in itr {
        if curr_date == None || entry.stamp.date_naive() != curr_date.unwrap() {
            curr_date = Some(entry.stamp.date_naive());
            buf += &format!("----DATE: {}----\n", entry.stamp.date());
        }
        let mut tmpl = " ";
        if entry.channel_id == 0 {
            tmpl = " says, ";
        }
        buf += &format!("[{:02}:{:02}] {}{}{}\n", entry.stamp.time().hour(), entry.stamp.time().minute(), entry.speaker, tmpl, entry.message);
    }
    return buf;
}

pub fn render_orig(itr: impl Iterator<Item=Logentry> + Send) -> String {
    let mut retval = String::from("");
    // for logentry in logentries {
    //     retval += 
    // }
    return retval;
}
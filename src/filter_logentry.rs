use crate::parse_logfile::*;
use chrono::{DateTime, FixedOffset};
use rayon::prelude::*;
use regex::Regex;



pub fn filter_channels(invec: Logfiles, channels: &Vec<i32>) -> Logfiles
{
    return invec.into_par_iter().filter(|d| channels.contains(&d.channel_id)).collect();
}

pub fn filter_dates(invec: Logfiles, date_pairs: impl Iterator<Item = (DateTime<FixedOffset>, DateTime<FixedOffset>)> + std::marker::Sync + std::marker::Send) -> Logfiles
{
    let dpc: Vec<_> = date_pairs.collect();
    return invec.into_par_iter().filter(|d| {
        //println!("stamp={}", &d.stamp);
        dpc.iter().any(|(start_date, end_date)| {
            //println!("start_date={}, end_date={}", start_date, end_date);
            &d.stamp >= &start_date && &d.stamp <= &end_date
        })
    }).collect();
}

pub fn filter_regex(invec: Logfiles, rx: Regex) -> Logfiles
{
    return invec.into_par_iter().filter(|d| rx.is_match(d.message.as_parallel_string())).collect();
}
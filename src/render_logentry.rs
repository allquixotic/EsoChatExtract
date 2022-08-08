use crate::parse_logfile::Logentry;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEF_PALETTE: Vec<String> = vec!["002b36", "073642", "586e75", "657b83", "839496", "93a1a1", "eee8d5", "fdf6e3", "b58900", "cb4b16", "dc322f", "d33682", "6c71c4", "268bd2", "2aa198", "859900"].iter().map(|&x| x.to_string()).collect();
}

fn render_bbcode(itr: impl Iterator<Item=Logentry> + Send) -> String {
    todo!();
    //let mut buf = String::from("");
    //return buf;
}
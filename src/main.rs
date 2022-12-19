pub mod parse_logfile;
pub mod render_logentry;
pub mod filter_logentry;
use chrono::{FixedOffset, Utc, TimeZone, Duration};
use filter_logentry::{filter_dates, filter_channels};
use tracing::{subscriber, error};
use tracing_subscriber::fmt::Subscriber;
use std::{collections::BTreeMap, fs::File, io::Write};
use surrealdb::{Datastore, Session, sql::Value, sql::{Strand, Number}};
use futures::executor;

use crate::render_logentry::render_plain;

fn main() {
    subscriber::set_global_default(Subscriber::default()).unwrap();
    let fils = parse_logfile::find_logfiles(&parse_logfile::DEFAULT_LOG_DIR.to_str().unwrap());
    let itr = &mut fils.into_iter();
    let channels = vec![0, 6];
    
    match parse_logfile::parse_all(itr) {
        Ok(logentries) => {

            /*
            let ds = executor::block_on(Datastore::new("file://./chatlog.db")).unwrap();

            //USE NS chatlog DB chatlog; DEFINE TABLE chatlog SCHEMAFULL; DEFINE FIELD stamp ON TABLE chatlog TYPE string; DEFINE FIELD channel_id ON TABLE chatlog TYPE string; DEFINE FIELD speaker ON TABLE chatlog TYPE string; DEFINE FIELD message ON TABLE chatlog TYPE string; DEFINE INDEX logline ON TABLE chatlog COLUMNS stamp, channel_id, speaker, message UNIQUE;
            
            //Create the table
            let ses = Session::for_kv();
            let preamble = "USE NS chatlog DB chatlog;";
            let table_creation_query = preamble.to_owned() + "DEFINE TABLE chatlog SCHEMAFULL;
            DEFINE FIELD stamp ON TABLE chatlog TYPE string;
            DEFINE FIELD channel_id ON TABLE chatlog TYPE string;
            DEFINE FIELD speaker ON TABLE chatlog TYPE string;
            DEFINE FIELD message ON TABLE chatlog TYPE string;
            DEFINE INDEX logline ON TABLE chatlog COLUMNS stamp, channel_id, speaker, message UNIQUE;";
            executor::block_on(ds.execute(&table_creation_query, &ses, None, false)).unwrap();
            let insert_template = preamble.to_owned() + "INSERT IGNORE INTO chatlog (stamp, channel_id, speaker, message) VALUES ($stamp, $channel_id, $speaker, $message);";

            for logentry in logentries
            {
                let mut btr: BTreeMap<String, Value> = BTreeMap::new();
                btr.insert("stamp".to_string(), Value::Strand(Strand(logentry.stamp.to_string())));
                btr.insert("channel_id".to_string(), Value::Number(Number::Int(logentry.channel_id.into())));
                btr.insert("speaker".to_string(), Value::Strand(Strand(logentry.speaker.to_string())));
                btr.insert("message".to_string(), Value::Strand(Strand(logentry.message.to_string())));
                let mp = Some(btr);
                match executor::block_on(ds.execute(&insert_template, &ses, mp, false)) {
                    Ok(_) => (),
                    Err(e) => println!("Error: {}", e),
                }
            }
            */
            let fo = FixedOffset::west(5*3600);
            //let fo = FixedOffset::west(0);
            let date_vec = vec![
                (1, 18), 
                (1, 25),
                (2, 1),
                (2, 15),
                (2, 22),
                (3, 8),
                (3, 22),
                (3, 29),
                (4, 5),
                (4, 12),
                (4, 19),
                (5, 3),
                (5, 10),
                (5, 17),
                (5, 24),
                (6, 7),
                (6, 14),
                (6, 21),
                (6, 28),
                (7, 19),
                (7, 26),
                (8, 2),
                (8, 9),
                (8, 23),
                (8, 30),
                (9, 13),
                (9, 20),
                (9, 27),
                (10, 11),
                (10, 25),
                (11, 1),
                (11, 15),
                (11, 22),
                (12, 6),
                (12, 13)];
            let ymds = date_vec.iter().map(|(month, day)| (
                Utc.ymd(2022, *month, *day).and_hms(20, 0, 0).checked_add_signed(Duration::hours(5)).unwrap().with_timezone(&fo), 
                Utc.ymd(2022, *month, *day).and_hms(21, 5, 0).checked_add_signed(Duration::hours(5)).unwrap().with_timezone(&fo)));
            let date_filtered = 
                filter_dates(filter_channels(logentries, &channels), ymds);

            let p = render_plain(date_filtered.into_iter());

            let fo = File::create("tp_logs.txt");
            fo.unwrap().write_all(p.as_bytes()).unwrap();

            // let mut people: BTreeSet<String> = BTreeSet::new();

            // for entry in date_filtered
            // {
            //     people.insert(entry.speaker);
            // }

            // for person in people
            // {
            //     println!("{}", person);
            // }

            // let rx = Regex::new(r"(?i).*searchterm1.*searchterm2.*").unwrap();
            // for entry in filter_regex(date_filtered, rx) 
            // {
            //     println!("{}: {}", entry.speaker, entry.message);
            // }
        },
        Err(e) => error!("{}", e),
    }
}

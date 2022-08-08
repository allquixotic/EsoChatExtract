pub mod parse_logfile;
pub mod render_logentry;
use rayon::prelude::*;
use tracing::{info,subscriber, error};
use tracing_subscriber::fmt::Subscriber;

fn main() {
    subscriber::set_global_default(Subscriber::default()).unwrap();
    let fils = parse_logfile::find_logfiles(&parse_logfile::DEFAULT_LOG_DIR.to_str().unwrap());
    let itr = &mut fils.into_iter();
    let channels = vec![1];
    match parse_logfile::parse_all(itr) {
        Ok(logentries) => {

            let a = logentries.iter();
            let b = a.par_bridge();
            let c = b.filter(|d| channels.contains(&d.value().channel_id));
            
            c.for_each(|x| {
                info!("{:?}", x.value());
            });
        },
        Err(e) => error!("{}", e),
    }
}

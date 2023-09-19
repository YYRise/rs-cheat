mod str_usage;
mod internal;

use docopt::Docopt;
use crate::internal::config::Config;

fn main() {
    println!("Hello, world!");
    let s = str_usage::usage();
    // println!("{}", s);
    let args = Docopt::new(s)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    println!("  init: {}, {}", args.get_str("--init"), args.get_bool("--init"));
    let conf_path = String::from("conf/conf.yml");
    let conf = Config::new(conf_path.as_str(), false).unwrap();
    println!("{:?}", conf);
}

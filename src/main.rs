mod str_usage;
use docopt::Docopt;

fn main() {
    println!("Hello, world!");
    let s = str_usage::usage();
    // println!("{}", s);
    let args = Docopt::new(s)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    println!("  init: {}, {}", args.get_str("--init"), args.get_bool("--init"));
}

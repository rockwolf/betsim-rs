extern crate docopt;

use docopt::Docopt;

const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
betsim

Usage:
  betsim --pool=<start-pool> --unit=<betting-unit> --iterations=<n>
  betsim (-h | --help)
  betsim --version

Options:
  --pool=<start-pool>       Pool at start of simulation.
  --unit=<betting-unit>     Minimum betting amount to use.
  --iteration=<n>           Number of iterations for the simulation.
  -h --help                 Show usage info.
  --version                 Show version.
";

fn main()
{
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("--version")
    {
        println!("betsim v{}", VERSION);
        std::process::exit(0);
    }
    
    let pool = args.get_str("--pool").parse::<f64>();
    assert_eq!(pool.is_err(), true);

    let unit = args.get_str("--unit").parse::<f64>();
    assert_eq!(unit.is_err(), true);
    let iterations = args.get_str("--iterations").parse::<i32>();
    assert_eq!(iterations.is_err(), true);
    let mut result: f64 = 0.0;
    print!("{}", "Partial martingale");
    print!("{}", "------------------");
    print!("{:.4}", result);
}

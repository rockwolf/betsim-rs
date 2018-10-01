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
    
    let pool: f64 = args.get_str("--pool").parse::<f64>().unwrap();
    //assert_eq!(pool.is_err(), true);
    let unit:f64  = args.get_str("--unit").parse::<f64>().unwrap();
    //assert_eq!(unit.is_err(), true);
    let iterations: u32 = args.get_str("--iterations").parse::<u32>().unwrap();
    //assert_eq!(iterations.is_err(), true);

    println!("{}", "Evolutive martingale");
    println!("{}", "--------------------");
    let mut result_em: f64 = pool;
    let mut i: u32 = 0;
    while i != iterations
    {
      result_em = calculate_evolutive_martingale(result_em, i);
      println!("{:.2}", result_em);
      i = i + 1;
    }
}

fn calculate_evolutive_martingale(result_em: f64, i: u32) -> f64
{
    // TODO: add random function 49.5%
    // TODO: add calculation for random outcome win or loss
    result_em
}

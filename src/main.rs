extern crate docopt;
extern crate rand;

use docopt::Docopt;
use rand::Rng;

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
    let unit_base: f64 = args.get_str("--unit").parse::<f64>().unwrap();
    let iterations: u32 = args.get_str("--iterations").parse::<u32>().unwrap();

    println!("{}", "Evolutive martingale");
    println!("{}", "--------------------");
    let mut result_em: f64 = pool;
    let mut unit_current: f64 = unit_base;
    let mut i: u32 = 0;
    while i != iterations
    {
        result_em = calculate_evolutive_martingale(result_em, i, unit_base, &mut unit_current);
        if result_em >= 0.0
        {
            println!("{:.2}", result_em);
        }
        i = i + 1;
    }
}

fn calculate_evolutive_martingale(
    result_em: f64,
    i: u32,
    unit_base: f64,
    unit_current: &mut f64,
) -> f64
{
    let mut result: f64 = 0.0;
    if is_win()
    {
        result = result_em - *unit_current;
        if *unit_current < unit_base
        {
            // TODO: unit_base needs can be uneven?
            *unit_current = *unit_current / 2.0;
        }
    }
    else
    {
        result = result_em + *unit_current;
        // TODO: Make 16.0 aka max_unit a parameter.
        if *unit_current < 16.0
        {
            *unit_current = *unit_current + unit_base;
        }
        else
        {
            // TODO: experiment with making this half unit_current?
            *unit_current = unit_base;
        }
    }
    result
}

fn is_win() -> bool
{
    // TODO: Make 5% casino advantage a parameter.
    rand::thread_rng().gen_bool(0.495)
}

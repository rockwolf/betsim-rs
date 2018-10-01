extern crate docopt;
extern crate rand;

use docopt::Docopt;
use rand::Rng;

const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
betsim

Usage:
  betsim --pool=<start-pool> --unit=<betting-unit> --iterations=<n> (--em | --eam) [--verbose]
  betsim (-h | --help)
  betsim --version

Options:
  --pool=<start-pool>       Pool at start of simulation.
  --unit=<betting-unit>     Minimum betting amount to use.
  --iteration=<n>           Number of iterations for the simulation.
  --em                      Evolutive martingale.
  --eam                     Evalutive anti-martingale.
  --verbose                 Print iteration details.
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

    if args.get_bool("--em")
    {
        println!("{}", "Evolutive martingale");
        println!("{}", "--------------------");
    }
    else if args.get_bool("--eam")
    {
        println!("{}", "Evolutive anti-martingale");
        println!("{}", "-------------------------");
    }
    let mut result_calc: f64 = pool;
    let mut unit_current: f64 = unit_base;
    let mut i: u32 = 0;
    while i != iterations
    {
        if args.get_bool("--em")
        {
            result_calc = calculate_evolutive_martingale(result_calc, unit_base, &mut unit_current);
        }
        else if args.get_bool("--eam")
        {
            result_calc =
                calculate_evolutive_anti_martingale(result_calc, unit_base, &mut unit_current);
        }
        if result_calc >= 0.0
        {
            if args.get_bool("--verbose")
            {
                println!("{:.2}", result_calc);
            }
        }
        else
        {
            println!("WARNING! Evolved to 0.0!");
            break;
        }
        i = i + 1;
    }
    println!("Total number of iterations: {}.", i);
    if result_calc > pool
    {
        println!("System succeeded!");
    }
    else
    {
        println!("System failed!");
    }
}

fn calculate_evolutive_martingale(result_calc: f64, unit_base: f64, unit_current: &mut f64) -> f64
{
    let result: f64;
    if is_win()
    {
        result = result_calc + *unit_current;
        if *unit_current > unit_base
        {
            // TODO: unit_base can be uneven?
            *unit_current = *unit_current / 2.0;
        }
        else
        {
            *unit_current = unit_base;
        }
    }
    else
    {
        result = result_calc - *unit_current;
        // TODO: Make 16.0 aka max_unit a parameter.
        if *unit_current < 16.0
        {
            *unit_current = *unit_current + unit_base;
        }
        else
        {
            // TODO: experiment with making this half unit_current?
            //*unit_current = *unit_current / 2.0;
            *unit_current = unit_base;
        }
    }
    result
}

fn calculate_evolutive_anti_martingale(
    result_calc: f64,
    unit_base: f64,
    unit_current: &mut f64,
) -> f64
{
    let result: f64;
    if is_win()
    {
        result = result_calc + *unit_current;
        // TODO: Make 3 a parameter? MAX_ANTI_MG_RUN
        if *unit_current < 2.0 * unit_base
        {
            *unit_current = *unit_current + unit_base;
        }
        else
        {
            // Note: Take profits after 3 wins.
            *unit_current = unit_base;
        }
    }
    else
    {
        result = result_calc - *unit_current;
        // TODO: experiment with resetting to unit_base?
        *unit_current = *unit_current / 2.0;
    }
    result
}

fn is_win() -> bool
{
    // TODO: Make 5% casino advantage a parameter.
    rand::thread_rng().gen_bool(0.495)
}

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "arguments", about = "Get range of integers to find primes in")]
struct Opt {
    /// Set lower end of range of integers
    // short and long flags (-s, --start) will be deduced from field name
    #[structopt(short, long)]
    start: u32,

    /// Set upper end of range of integers
    // short and long flags (-e, --end) will be deduced from field name
    #[structopt(short, long)]
    end: u32
}

fn main()
{
    let opt = Opt::from_args();
    println!("{:?}", opt);
    let start = opt.start;
    let end = opt.end;
    println!("start: {}", start);
    println!("end: {}", end);
}

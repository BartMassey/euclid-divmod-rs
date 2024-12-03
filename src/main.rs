use clap::Parser;

extern crate edm;

#[derive(Parser)]
struct Args {
    op: String,
    numerator: f32,
    denominator: f32,
}

fn fail(e: Box<dyn std::error::Error>) -> ! {
    eprintln!("{}", e);
    std::process::exit(1);
}

fn main() {
    let args = Args::parse();
    let op = edm::get_op(&args.op).unwrap_or_else(|e| fail(e.into()));
    let (q, r) = op(args.numerator, args.denominator);
    println!("{:20?} {:20?}", q, r);
}

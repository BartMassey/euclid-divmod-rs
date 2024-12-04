use clap::Parser;

extern crate edm;

#[derive(Parser)]
struct Args {
    #[arg(long, short, action)]
    quiet: bool,
    #[arg(long, name="all-signs", action)]
    all_signs: bool,
    op: String,
    numerator: f32,
    denominator: f32,
}

fn fail(e: Box<dyn std::error::Error>) -> ! {
    eprintln!("{}", e);
    std::process::exit(1);
}

fn report(op: edm::D, n: f32, d: f32, quiet: bool) {
    let (q, r) = op(n, d);
    let reconstructed = q * d + r == n;
    let in_range = r >= 0.0 && r < d.abs();
    let output = !quiet || !reconstructed || !in_range;
    if output {
        print!("{} {} → {} {}", n, d, q, r);
    }
    if !reconstructed {
        print!(" ({}×{}+{}≠{}, d={})", q, d, r, n, n - q * d + r);
    }
    if !in_range {
        let delta = if r < 0.0 {
            r
        } else {
            r - d
        };
        print!("({} not in [0..{})), d={})", r, d.abs(), delta);
    }
    if output {
        println!();
    }
}

fn main() {
    let args = Args::parse();
    let op = edm::get_op(&args.op).unwrap_or_else(|e| fail(e.into()));
    report(op, args.numerator, args.denominator, args.quiet);
    if args.all_signs {
        report(op, -args.numerator, args.denominator, args.quiet);
        report(op, args.numerator, -args.denominator, args.quiet);
        report(op, -args.numerator, -args.denominator, args.quiet);
    }
}

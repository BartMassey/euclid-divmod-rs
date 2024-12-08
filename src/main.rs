use clap::Parser;

use edm::float_utils::DisplayFloat as DF;

#[derive(Parser)]
struct Args {
    #[arg(long, short, action)]
    quiet: bool,
    #[arg(long, name="all-signs", action)]
    all_signs: bool,
    #[arg(long, name="display-float", action)]
    display_float: bool,
    op: String,
    numerator: f32,
    denominator: f32,
}

fn fail(e: Box<dyn std::error::Error>) -> ! {
    eprintln!("{}", e);
    std::process::exit(1);
}

fn report(op: edm::D, n: f32, d: f32, quiet: bool, df: bool) {
    let (q, r) = op(n, d);
    let reconstruction = f32::mul_add(q, d, r);
    let reconstructed = reconstruction == n;
    let in_range = r >= 0.0 && r < d.abs();
    let output = !quiet || !reconstructed || !in_range;
    if output {
        if df {
            print!("{} {} → {} {}", DF(n), DF(d), DF(q), DF(r));
        } else {
            print!("{} {} → {} {}", n, d, q, r);
        }
    }
    if !reconstructed {
        if df {
            print!(" ({}×{}+{}≠{}, Δ={})", DF(q), DF(d), DF(r), DF(n), DF(n - reconstruction));
        } else {
            print!(" ({}×{}+{}≠{}, Δ={})", q, d, r, n, n - reconstruction);
        }
    }
    if !in_range {
        let delta = if r < 0.0 {
            r
        } else {
            r - d
        };
        if df {
            print!(" ({}∉[0..{}), Δ={})", DF(r), DF(d.abs()), DF(delta));
        } else {
            print!(" ({}∉[0..{}), Δ={})", r, d.abs(), delta);
        }
    }
    if output {
        println!();
    }
}

fn main() {
    let args = Args::parse();
    let op = edm::get_op(&args.op).unwrap_or_else(|e| fail(e.into()));
    report(op, args.numerator, args.denominator, args.quiet, args.display_float);
    if args.all_signs {
        report(op, -args.numerator, args.denominator, args.quiet, args.display_float);
        report(op, args.numerator, -args.denominator, args.quiet, args.display_float);
        report(op, -args.numerator, -args.denominator, args.quiet, args.display_float);
    }
}

use std::env;
use std::f64;
use std::process;

use getopts::Options;
use rand::Rng;
use rand_pcg::Mcg128Xsl64;
use tsp_sa_meta::DistType;
use tsp_sa_meta::{Tour, TownDistance};

#[derive(Debug)]
struct Args {
    seed: u128,

    towns: usize,
    box_size: f64,
    dist: DistType,
    dim: usize,

    temp_max: f64,
    temp_min: f64,
    temp_step: f64,
    sample_num: usize,
}

fn print_usage(program: &str, opts: &Options) -> ! {
    let brief = format!("Usage: {}", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

fn parse_args() -> Args {
    let args: Vec<_> = env::args().collect();
    let mut opt = Options::new();
    opt.optflag("h", "help", "print this help menu");
    opt.optopt("s", "seed", "random number's seed(required)", "SEED");
    opt.optopt("t", "towns", "the number of town(required)", "TOWNS");
    opt.optopt("l", "box-size", "box size(required)", "SIZE");
    opt.optopt("d", "dist", "distance definition(default L2)", "DIST");
    opt.optopt("i", "dim", "dimension of position(default 2)", "DIM");
    opt.optopt("M", "temp-max", "max temperature(default 100)", "T0");
    opt.optopt("m", "temp-min", "min temperature(default 0.5)", "T1");
    opt.optopt("p", "temp-step", "temperature change step(0.5)", "dT");
    opt.optopt("c", "sample-num", "sample num(default 10^4)", "COUNT");
    let m = opt
        .parse(&args[1..])
        .unwrap_or_else(|f| panic!(f.to_string()));

    if m.opt_present("h") {
        print_usage(&args[0], &opt);
    }
    if !m.free.is_empty() {
        print_usage(&args[0], &opt);
    }
    Args {
        seed: m
            .opt_str("seed")
            .unwrap_or_else(|| print_usage(&args[0], &opt))
            .parse::<u128>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        towns: m
            .opt_str("towns")
            .unwrap_or_else(|| print_usage(&args[0], &opt))
            .parse::<usize>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        box_size: m
            .opt_str("box-size")
            .unwrap_or_else(|| print_usage(&args[0], &opt))
            .parse::<f64>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        dist: m
            .opt_str("dist")
            .unwrap_or_else(|| "L2".to_string())
            .parse::<DistType>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        dim: m
            .opt_str("dim")
            .unwrap_or_else(|| "2".to_string())
            .parse::<usize>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        temp_max: m
            .opt_str("temp-max")
            .unwrap_or_else(|| "100".to_string())
            .parse::<f64>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        temp_min: m
            .opt_str("temp-min")
            .unwrap_or_else(|| "0.5".to_string())
            .parse::<f64>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        temp_step: m
            .opt_str("temp-step")
            .unwrap_or_else(|| "0.5".to_string())
            .parse::<f64>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        sample_num: m
            .opt_str("sample-num")
            .unwrap_or_else(|| "10000".to_string())
            .parse::<usize>()
            .unwrap_or_else(|f| panic!(f.to_string())),
    }
}

fn make_town<R: Rng>(random: &mut R, args: &Args) -> Tour {
    let mut town_pos = Vec::with_capacity(args.towns);
    for _ in 0..args.towns {
        let mut pos = Vec::with_capacity(args.dim);
        for _ in 0..args.dim {
            pos.push(random.gen_range(0.0, args.box_size));
        }
        town_pos.push(pos);
    }
    let town = TownDistance::new(&town_pos, args.dist);
    let tour = Tour::with_random(town, random);
    tour
}

fn mc<R: Rng>(random: &mut R, n: usize, tour: &mut Tour, beta: f64) {
    for _ in 0..n {
        let a = random.gen_range(0, n);
        let b = (a + random.gen_range(1, n)) % n;
        let delta = tour.try_2opt(a, b);
        if delta < 0.0 || random.gen_range(0.0, 1.0) < (-delta * beta).exp() {
            tour.do_2opt(a, b);
        }
    }
}

fn main() {
    let args = parse_args();
    let mut random = Mcg128Xsl64::new(args.seed);
    let mut tour = make_town(&mut random, &args);

    let norm_fact = args.towns as f64 * args.box_size;
    let mut temp = args.temp_max;
    for _ in 0..args.sample_num {
        mc(&mut random, args.towns, &mut tour, 1.0 / temp);
    }
    while temp >= args.temp_min {
        let mut s1 = 0.0;
        let mut s2 = 0.0;
        for _ in 0..args.sample_num {
            mc(&mut random, args.towns, &mut tour, 1.0 / temp);
            let energy = tour.get_total_dist();
            s1 += energy;
            s2 += energy * energy;
        }
        s1 /= args.sample_num as f64 * norm_fact;
        s2 /= args.sample_num as f64 * norm_fact * norm_fact;
        println!("{} {} {}", temp, s1, s2);

        temp -= args.temp_step;
    }
}

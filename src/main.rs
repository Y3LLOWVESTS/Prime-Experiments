use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::process;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply,
    Divide,
}

impl Operation {
    fn as_str(&self) -> &'static str {
        match self {
            Operation::Multiply => "multiply",
            Operation::Divide => "divide",
        }
    }

    fn parse(input: &str) -> Result<Self, String> {
        match input.trim().to_ascii_lowercase().as_str() {
            "multiply" | "mul" | "*" => Ok(Operation::Multiply),
            "divide" | "div" | "/" => Ok(Operation::Divide),
            other => Err(format!(
                "Invalid operation '{other}'. Use multiply or divide."
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Floor,
    Exact,
}

impl Mode {
    fn as_str(&self) -> &'static str {
        match self {
            Mode::Floor => "floor",
            Mode::Exact => "exact",
        }
    }

    fn parse(input: &str) -> Result<Self, String> {
        match input.trim().to_ascii_lowercase().as_str() {
            "floor" | "int" | "integer" | "truncate" => Ok(Mode::Floor),
            "exact" | "fraction" | "rational" => Ok(Mode::Exact),
            other => Err(format!(
                "Invalid mode '{other}'. Use floor or exact."
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rational {
    num: u128,
    den: u128,
}

impl Rational {
    fn new(num: u128, den: u128) -> Result<Self, String> {
        if den == 0 {
            return Err("Denominator cannot be zero.".to_string());
        }

        let g = gcd(num, den);
        Ok(Self {
            num: num / g,
            den: den / g,
        })
    }

    fn reciprocal(self) -> Result<Self, String> {
        if self.num == 0 {
            return Err("Cannot divide by zero.".to_string());
        }
        Rational::new(self.den, self.num)
    }

    fn apply_floor(self, n: u128) -> Result<u128, String> {
        let top = n
            .checked_mul(self.num)
            .ok_or_else(|| "Overflow while computing floor result.".to_string())?;
        Ok(top / self.den)
    }

    fn apply_exact(self, n: u128) -> Result<Rational, String> {
        let top = n
            .checked_mul(self.num)
            .ok_or_else(|| "Overflow while computing exact result.".to_string())?;
        Rational::new(top, self.den)
    }

    fn decimal_string(&self, digits: usize) -> String {
        let whole = self.num / self.den;
        let mut remainder = self.num % self.den;

        if digits == 0 {
            return whole.to_string();
        }

        let mut frac = String::with_capacity(digits);
        for _ in 0..digits {
            remainder *= 10;
            let digit = remainder / self.den;
            frac.push(std::char::from_digit(digit as u32, 10).unwrap());
            remainder %= self.den;
        }

        format!("{whole}.{frac}")
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{} (~{})", self.num, self.den, self.decimal_string(6))
        }
    }
}

#[derive(Debug)]
struct Config {
    start: u64,
    end: u64,
    factor_input: String,
    operation: Operation,
    mode: Mode,
    primes_only: bool,
    no_table: bool,
}

impl Config {
    fn default() -> Self {
        Self {
            start: 1,
            end: 115,
            factor_input: "0.625".to_string(),
            operation: Operation::Divide,
            mode: Mode::Floor,
            primes_only: false,
            no_table: false,
        }
    }

    fn usage() -> &'static str {
        r#"Usage:
  cargo run -- --start 1 --end 115 --op divide --factor 0.625 --mode floor
  cargo run -- --start 1 --end 115 --op multiply --factor 1.6 --mode floor
  cargo run -- --start 1 --end 115 --op multiply --factor 0.625 --mode exact --primes-only

Options:
  --start <n>         Starting integer (default: 1)
  --end <n>           Ending integer, inclusive (default: 115)
  --op <name>         divide | multiply  (default: divide)
  --factor <value>    Decimal, integer, or fraction. Examples:
                      0.625
                      1.6
                      5/8
                      8/5
  --mode <name>       floor | exact      (default: floor)
  --primes-only       Show only prime rows in the table
  --no-table          Skip the row-by-row table and print summaries only
  -h, --help          Show this help

Notes:
  - divide by 0.625 is equivalent to multiply by 1.6
  - divide by 1.6 is equivalent to multiply by 0.625
  - floor mode reproduces the integer-style jump experiment
"#
    }

    fn from_env_args() -> Result<Self, String> {
        let mut cfg = Config::default();
        let args: Vec<String> = env::args().skip(1).collect();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--help" => {
                    println!("{}", Config::usage());
                    process::exit(0);
                }
                "--start" => {
                    i += 1;
                    let value = args
                        .get(i)
                        .ok_or_else(|| "Missing value after --start".to_string())?;
                    cfg.start = value
                        .parse::<u64>()
                        .map_err(|_| format!("Invalid start value '{value}'"))?;
                }
                "--end" => {
                    i += 1;
                    let value = args
                        .get(i)
                        .ok_or_else(|| "Missing value after --end".to_string())?;
                    cfg.end = value
                        .parse::<u64>()
                        .map_err(|_| format!("Invalid end value '{value}'"))?;
                }
                "--op" => {
                    i += 1;
                    let value = args
                        .get(i)
                        .ok_or_else(|| "Missing value after --op".to_string())?;
                    cfg.operation = Operation::parse(value)?;
                }
                "--factor" => {
                    i += 1;
                    let value = args
                        .get(i)
                        .ok_or_else(|| "Missing value after --factor".to_string())?;
                    cfg.factor_input = value.clone();
                }
                "--mode" => {
                    i += 1;
                    let value = args
                        .get(i)
                        .ok_or_else(|| "Missing value after --mode".to_string())?;
                    cfg.mode = Mode::parse(value)?;
                }
                "--primes-only" => {
                    cfg.primes_only = true;
                }
                "--no-table" => {
                    cfg.no_table = true;
                }
                other => {
                    return Err(format!(
                        "Unknown argument '{other}'.\n\n{}",
                        Config::usage()
                    ));
                }
            }
            i += 1;
        }

        if cfg.start == 0 {
            return Err("Please use a start value of 1 or greater.".to_string());
        }

        if cfg.end < cfg.start {
            return Err("end must be greater than or equal to start.".to_string());
        }

        Ok(cfg)
    }
}

#[derive(Debug)]
struct Row {
    n: u64,
    is_prime: bool,
    floor_value: u128,
    exact_value: Rational,
    jump_from_prev: Option<u128>,
    prime_pair: Option<(u128, u128)>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let config = Config::from_env_args()?;
    let requested_factor = parse_rational(&config.factor_input)?;

    let effective_factor = match config.operation {
        Operation::Multiply => requested_factor,
        Operation::Divide => requested_factor.reciprocal()?,
    };

    let rows = build_rows(&config, effective_factor)?;

    println!("Prime experiment");
    println!("range: {}..={}", config.start, config.end);
    println!(
        "requested operation: {} {}",
        config.operation.as_str(),
        requested_factor
    );
    println!("effective transform on n: n * {}", effective_factor);
    println!("mode: {}", config.mode.as_str());
    println!();

    if !config.no_table {
        print_table(&rows, config.mode, config.primes_only);
        println!();
    }

    print_prime_pair_summary(&rows);

    Ok(())
}

fn build_rows(config: &Config, factor: Rational) -> Result<Vec<Row>, String> {
    let limit = config.end as usize;
    let primes = sieve(limit);

    let mut floor_values = vec![0_u128; limit + 1];
    let zero = Rational::new(0, 1)?;
    let mut exact_values = vec![zero; limit + 1];

    for n in 1..=config.end {
        let idx = n as usize;
        floor_values[idx] = factor.apply_floor(n as u128)?;
        exact_values[idx] = factor.apply_exact(n as u128)?;
    }

    let mut rows = Vec::with_capacity((config.end - config.start + 1) as usize);

    for n in config.start..=config.end {
        let idx = n as usize;

        let jump_from_prev = Some(floor_values[idx] - floor_values[idx - 1]);

        let prime_pair = if primes[idx] && n < config.end {
            Some((
                floor_values[idx] - floor_values[idx - 1],
                floor_values[idx + 1] - floor_values[idx],
            ))
        } else {
            None
        };

        rows.push(Row {
            n,
            is_prime: primes[idx],
            floor_value: floor_values[idx],
            exact_value: exact_values[idx],
            jump_from_prev,
            prime_pair,
        });
    }

    Ok(rows)
}

fn print_table(rows: &[Row], mode: Mode, primes_only: bool) {
    println!(
        "{:>6} {:>7} {:>22} {:>12} {:>12}",
        "n", "prime", "value", "jump_in", "pair"
    );
    println!("{}", "-".repeat(67));

    for row in rows {
        if primes_only && !row.is_prime {
            continue;
        }

        let prime_mark = if row.is_prime { "yes" } else { "" };
        let value = match mode {
            Mode::Floor => row.floor_value.to_string(),
            Mode::Exact => row.exact_value.to_string(),
        };

        let jump_in = row
            .jump_from_prev
            .map(|v| v.to_string())
            .unwrap_or_else(String::new);

        let pair = row
            .prime_pair
            .map(|(a, b)| format!("{a} {b}"))
            .unwrap_or_else(String::new);

        println!(
            "{:>6} {:>7} {:>22} {:>12} {:>12}",
            row.n, prime_mark, value, jump_in, pair
        );
    }
}

fn print_prime_pair_summary(rows: &[Row]) {
    let mut pairs = Vec::new();
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();

    for row in rows {
        if let Some((a, b)) = row.prime_pair {
            let pair = format!("{a} {b}");
            pairs.push(pair.clone());
            *counts.entry(pair).or_insert(0) += 1;
        }
    }

    println!("Prime jump-pair sequence:");
    if pairs.is_empty() {
        println!("(no prime pairs available in this range)");
    } else {
        println!("{}", pairs.join(", "));
    }

    println!();
    println!("Pair counts:");
    if counts.is_empty() {
        println!("(none)");
    } else {
        for (pair, count) in counts {
            println!("  {pair} -> {count}");
        }
    }
}

fn parse_rational(input: &str) -> Result<Rational, String> {
    let s = input.trim();

    if s.is_empty() {
        return Err("Factor cannot be empty.".to_string());
    }

    if s.starts_with('-') {
        return Err("Negative factors are not supported in this first version.".to_string());
    }

    if let Some((left, right)) = s.split_once('/') {
        let num = left
            .trim()
            .parse::<u128>()
            .map_err(|_| format!("Invalid fraction numerator in '{s}'"))?;
        let den = right
            .trim()
            .parse::<u128>()
            .map_err(|_| format!("Invalid fraction denominator in '{s}'"))?;
        return Rational::new(num, den);
    }

    if let Some((whole_part, frac_part)) = s.split_once('.') {
        let whole = if whole_part.is_empty() {
            0_u128
        } else {
            whole_part
                .parse::<u128>()
                .map_err(|_| format!("Invalid decimal whole part in '{s}'"))?
        };

        let frac_digits = frac_part.trim();
        if frac_digits.is_empty() {
            return Rational::new(whole, 1);
        }

        let frac_num = frac_digits
            .parse::<u128>()
            .map_err(|_| format!("Invalid decimal fractional part in '{s}'"))?;

        let mut den = 1_u128;
        for _ in 0..frac_digits.len() {
            den = den
                .checked_mul(10)
                .ok_or_else(|| format!("Decimal is too precise to parse safely: '{s}'"))?;
        }

        let top = whole
            .checked_mul(den)
            .and_then(|v| v.checked_add(frac_num))
            .ok_or_else(|| format!("Decimal is too large to parse safely: '{s}'"))?;

        return Rational::new(top, den);
    }

    let value = s
        .parse::<u128>()
        .map_err(|_| format!("Invalid factor '{s}'"))?;
    Rational::new(value, 1)
}

fn sieve(limit: usize) -> Vec<bool> {
    let mut prime = vec![true; limit + 1];

    prime[0] = false;
    if limit >= 1 {
        prime[1] = false;
    }

    let mut p = 2;
    while p * p <= limit {
        if prime[p] {
            let mut multiple = p * p;
            while multiple <= limit {
                prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }

    prime
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}
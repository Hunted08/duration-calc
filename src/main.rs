use clap::Parser;
use std::io::{self, Read};

mod calculator;
mod parser;

use calculator::Calculator;
use parser::DurationParser;

#[derive(Parser)]
#[command(name = "duration-calc")]
#[command(about = "Calculate time durations with natural language input", long_about = None)]
struct Cli {
    #[arg(help = "Duration expression (e.g., '2h 30m + 45m')")]
    expression: Vec<String>,

    #[arg(short, long, help = "Output in total minutes")]
    minutes: bool,

    #[arg(short = 'H', long, help = "Output in total hours")]
    hours: bool,

    #[arg(short, long, help = "Output in ISO 8601 format (PT2H30M)")]
    iso: bool,
}

fn main() {
    let cli = Cli::parse();

    let expression = if cli.expression.is_empty() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer.trim().to_string()
    } else {
        cli.expression.join(" ")
    };

    if expression.is_empty() {
        eprintln!("Error: No expression provided");
        std::process::exit(1);
    }

    match calculate_and_format(&expression, &cli) {
        Ok(output) => println!("{}" , output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn calculate_and_format(expression: &str, cli: &Cli) -> Result<String, String> {
    let parser = DurationParser::new();
    let tokens = parser.parse(expression)?;
    
    let calculator = Calculator::new();
    let result = calculator.evaluate(tokens)?;

    if cli.minutes {
        Ok(format!("{} minutes", result.as_minutes()))
    } else if cli.hours {
        Ok(format!("{:.2} hours", result.as_hours()))
    } else if cli.iso {
        Ok(result.to_iso8601())
    } else {
        Ok(result.to_human_readable())
    }
}

# duration-calc

CLI calculator for time durations with natural language input like '2h 30m + 45m' for quick time arithmetic

## Features

- Parse natural language duration strings (e.g., '2h', '30m', '1d', '45s')
- Support basic arithmetic: addition (+), subtraction (-), multiplication (*), division (/)
- Handle compound durations in single expression (e.g., '2h 30m + 1h 15m')
- Multiple output formats: human-readable (2h 30m), total minutes, total hours, ISO 8601 (PT2H30M)
- Support for negative durations and proper overflow handling
- Chain multiple operations in one expression (e.g., '8h - 30m + 1h')
- Accept input from command line arguments or stdin for piping
- Precision handling for fractional results (e.g., '5h / 2 = 2h 30m')
- Error messages for invalid syntax or unsupported operations
- Support weeks (w), days (d), hours (h), minutes (m), seconds (s) units

## How to Use

Use this project when you need to:

- Quickly solve problems related to duration-calc
- Integrate rust functionality into your workflow
- Learn how rust handles common patterns

## Installation

```bash
# Clone the repository
git clone https://github.com/KurtWeston/duration-calc.git
cd duration-calc

# Install dependencies
cargo build
```

## Usage

```bash
cargo run
```

## Built With

- rust

## Dependencies

- `clap`
- `regex`
- `thiserror`

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

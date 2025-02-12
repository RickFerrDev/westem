use std::env::args;
use std::error::Error;
use std::process::exit;

/// Structure to store the operands and operator of the operation.
///
/// left: The left operand.
/// operator: The operator of the operation.
/// right: The right operand.
pub struct Config {
    pub left: f64,
    pub operator: String,
    pub right: f64,
}

/// Function to parse the command line arguments and return a Config struct.
///
/// args: An iterator over the command line arguments.
///
/// Returns: A Result containing the Config struct or an error.
pub fn parse_config(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
    // Ignore the first argument, which is the program name.
    args.next();

    // Get the left operand.
    let left = match args.next() {
        Some(value) => value.trim().parse()?,
        None => return Err(Box::from("Didn't get the left side of the operation")),
    };

    // Get the operator.
    let operator = match args.next() {
        Some(value) => value.trim().to_lowercase(), // Convert the operator to lowercase.
        None => return Err(Box::from("Didn't get the operator")),
    };

    // Get the right operand.
    let right = match args.next() {
        Some(value) => value.trim().parse()?,
        None => return Err(Box::from("Didn't get the right side of the operation")),
    };

    // Return the Config struct.
    Ok(Config {
        left,
        operator,
        right,
    })
}

/// Function to calculate the result of the operation.
///
/// config: A reference to the Config struct containing the operands and operator.
///
/// Returns: A Result containing the result of the operation or an error.
pub fn calculate(config: &Config) -> Result<f64, Box<dyn Error>> {
    // Perform the operation according to the operator.
    match config.operator.as_str() {
        "+" => Ok(config.left + config.right),
        "-" => Ok(config.left - config.right),
        "/" => Ok(config.left / config.right),
        "*" => Ok(config.left * config.right),
        _ => Err(Box::from("Invalid operation")),
    }
}

/// Main function of the program.
fn main() {
    // Get the Config struct from the command line arguments.
    let config = parse_config(args()).unwrap_or_else(|x| {
        // Print the error message and exit the program.
        eprintln!("{x}");
        exit(1)
    });

    // Calculate the result of the operation.
    let result = calculate(&config).unwrap_or_else(|x| {
        // Print the error message and exit the program.
        eprintln!("{x}");
        exit(1)
    });

    // Print the result.
    println!("{result}")
}

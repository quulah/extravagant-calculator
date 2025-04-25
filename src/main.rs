use std::env;

/// Calculates the offset in bytes for a given pixel coordinate in a 320x200 resolution VGA buffer.
///
/// The function takes the x and y coordinates as unsigned 16-bit integers.
/// It checks if the coordinates are within the valid range for the resolution.
/// If the coordinates are valid, it calculates the offset using the formula: `y * 320 + x`.
/// The result is returned as an `Option<u32>`.  If the coordinates are out of bounds,
/// an error message is printed to standard output, and `None` is returned.
///
/// # Arguments
///
/// * `x`: The x-coordinate of the pixel (0-319).
/// * `y`: The y-coordinate of the pixel (0-199).
///
/// # Returns
///
/// * `Option<u32>`: The offset in bytes as an unsigned 32-bit integer, wrapped in `Some()`.
///                    Returns `None` if the coordinates are out of bounds.
///
fn calculate_offset(x: u16, y: u16) -> Option<u32> {
    let width = 320;
    let height = 200;

    if x < width && y < height {
        Some((y * width + x).into())
    } else {
        println!("Error: Coordinates ({}, {}) are out of bounds (0-{}, 0-{}).", x, y, width - 1, height - 1);
        None
    }
}

/// Main entry point for the VGA offset calculator program.
///
/// This program takes two command-line arguments: the x and y coordinates of a pixel
/// in a 320x200 resolution VGA buffer. It parses these arguments as unsigned 16-bit
/// integers, calls the `calculate_offset` function to compute the memory offset,
/// and prints the result to standard output.
///
/// # Arguments
///
/// The program expects two command-line arguments:
///
/// * `<x>`: The x-coordinate of the pixel (0-319).
/// * `<y>`: The y-coordinate of the pixel (0-199).
///
/// # Usage with Cargo
/// #
/// # Compile optimized for release and run directly.
///
/// ```bash
/// cargo run --quiet --release <x> <y>
/// ```
///
/// For example, to calculate the offset for pixel (10, 20):
///
/// ```bash
/// cargo run --quiet --release 10 20
/// 6410
/// ```
///
/// # Errors
///
/// The program will print an error message to standard error in the following cases:
///
/// * If the number of command-line arguments is not 2.
/// * If the provided arguments cannot be parsed as unsigned 16-bit integers.
/// * If the provided coordinates are outside the valid range of 0-319 for x and 0-199 for y.
///
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <x> <y>", args[0]);
        return;
    }

    if let (Ok(x), Ok(y)) = (args[1].parse::<u16>(), args[2].parse::<u16>()) {
        let offset = calculate_offset(x, y);
        println!("{:?}", offset.unwrap());
    } else {
        println!("Error: Please provide two integer values for x and y.");
    }
}


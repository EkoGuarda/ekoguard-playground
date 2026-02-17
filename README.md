# ekoguard-playground

The place for some test cases and experimental implementations.

## Overview

This repository serves as a playground for testing various algorithms, data structures, and computational methods. It contains example implementations in different programming languages to demonstrate concepts and serve as reference implementations.

## Examples

### Monte Carlo Pi Calculation (Rust)
**Location**: `examples/monte-rust/`

A Rust implementation that calculates the value of Pi using the Monte Carlo method. This statistical approach approximates Pi by randomly sampling points in a square and determining how many fall within a quarter circle inscribed in that square.

#### How it works:
1. Generate random points (x, y) where both coordinates are between 0 and 1
2. Check if each point falls inside a quarter circle of radius 1 (i.e., if x² + y² ≤ 1)
3. The ratio of points inside the circle to total points approximates π/4
4. Multiply by 4 to get the approximation of π

#### Mathematical basis:
- Area of quarter circle = πr²/4 = π/4 (when r=1)
- Area of square = 1
- Ratio = (π/4) / 1 = π/4
- Therefore: π ≈ 4 × (points_inside_circle / total_points)

The accuracy improves as the number of random samples increases.

## Contributing

Feel free to add more examples and test cases to this repository. Each example should include:
- Clear documentation of what it demonstrates
- Instructions on how to build and run it
- Comments explaining the key concepts

## License

See [LICENSE](LICENSE) for details.

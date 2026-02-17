# Monte Carlo Pi Calculation

A Rust implementation that calculates the value of Pi using the Monte Carlo method.

## How it Works

The Monte Carlo method approximates Pi by randomly sampling points in a square and determining how many fall within a quarter circle inscribed in that square.

### Algorithm Steps:
1. Generate random points (x, y) where both coordinates are between 0 and 1
2. Check if each point falls inside a quarter circle of radius 1 (i.e., if x² + y² ≤ 1)
3. The ratio of points inside the circle to total points approximates π/4
4. Multiply by 4 to get the approximation of π

### Mathematical Basis:
- Area of quarter circle = πr²/4 = π/4 (when r=1)
- Area of square = 1
- Ratio = (π/4) / 1 = π/4
- Therefore: π ≈ 4 × (points_inside_circle / total_points)

The accuracy improves as the number of random samples increases.

## Building and Running

### Prerequisites
- Rust toolchain (install from https://rustup.rs/)

### Build
```bash
cd examples/monte-rust
cargo build --release
```

### Run
```bash
# Run with default 1 million samples
cargo run --release

# Run with custom number of samples (e.g., 10 million)
cargo run --release 10000000
```

## Example Output

```
Calculating Pi using Monte Carlo method with 1000000 samples...

Results:
  Estimated Pi: 3.1419880000
  Actual Pi:    3.1415926536
  Error:        0.012577%
```

## Performance Notes

The accuracy of the approximation increases with the square root of the number of samples:
- 10,000 samples: ~1% error
- 100,000 samples: ~0.3% error
- 1,000,000 samples: ~0.1% error
- 10,000,000 samples: ~0.03% error

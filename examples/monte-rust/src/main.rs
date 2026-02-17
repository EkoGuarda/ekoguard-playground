use rand::Rng;
use std::env;

/// Calculate Pi using the Monte Carlo method
///
/// This implementation randomly generates points in a unit square [0,1] x [0,1]
/// and checks if they fall within a quarter circle of radius 1.
///
/// Mathematical basis:
/// - Area of quarter circle = πr²/4 = π/4 (when r=1)
/// - Area of unit square = 1
/// - Ratio = (π/4) / 1 = π/4
/// - Therefore: π ≈ 4 × (points_inside_circle / total_points)
fn monte_carlo_pi(num_samples: u64) -> f64 {
    let mut rng = rand::thread_rng();
    let mut inside_circle = 0u64;

    for _ in 0..num_samples {
        // Generate random point (x, y) in [0, 1] x [0, 1]
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        // Check if point is inside the quarter circle (x² + y² ≤ 1)
        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }

    // Calculate Pi approximation
    4.0 * (inside_circle as f64) / (num_samples as f64)
}

fn main() {
    // Default to 1 million samples if not specified
    let num_samples = env::args()
        .nth(1)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(1_000_000);

    println!(
        "Calculating Pi using Monte Carlo method with {} samples...",
        num_samples
    );

    let pi_estimate = monte_carlo_pi(num_samples);
    let actual_pi = std::f64::consts::PI;
    let error = ((pi_estimate - actual_pi) / actual_pi).abs() * 100.0;

    println!("\nResults:");
    println!("  Estimated Pi: {:.10}", pi_estimate);
    println!("  Actual Pi:    {:.10}", actual_pi);
    println!("  Error:        {:.6}%", error);
}

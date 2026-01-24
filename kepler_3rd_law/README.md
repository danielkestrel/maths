# 🪐 Planet Distance Calculator

A command-line tool that calculates a planet's distance from the Sun using **Kepler's Third Law of Planetary Motion**.

Enter an orbital period → get the distance in meters, kilometers, and astronomical units.

## The Physics

Kepler's Third Law states that the square of a planet's orbital period is proportional to the cube of its distance from the Sun:

```
T² = (4π²/GM) × r³
```

Rearranging for distance:

```
r = ∛(GM × T² / 4π²)
```

Where:
- **T** = orbital period (seconds)
- **G** = gravitational constant (6.67 × 10⁻¹¹ N⋅m²/kg²)
- **M** = mass of the Sun (1.99 × 10³⁰ kg)
- **r** = orbital radius (distance from Sun)

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/kepler_3rd_law.git
cd kepler_3rd_law

# Build and install
cargo install --path .
```

## Usage

```bash
# Default unit is days
planet_distance 365.25

# Specify units explicitly
planet_distance 365.25 days
planet_distance 8766 hours
planet_distance 31557600 seconds
```

### Output

```
$ planet_distance 365.25
Orbital period: 365.25 days
Distance from sun: 1.496e+11 meters
                   1.496e+08 km
                   1.000 Astronomical Unit (AU)
```

## Examples

| Planet  | Orbital Period (days) | Command                  | Distance (AU) |
|---------|----------------------|--------------------------|---------------|
| Mercury | 88                   | `planet_distance 88`     | ~0.39         |
| Venus   | 225                  | `planet_distance 225`    | ~0.72         |
| Earth   | 365.25               | `planet_distance 365.25` | ~1.00         |
| Mars    | 687                  | `planet_distance 687`    | ~1.52         |
| Jupiter | 4333                 | `planet_distance 4333`   | ~5.20         |
| Saturn  | 10759                | `planet_distance 10759`  | ~9.54         |

## How It Works

The core calculation in Rust:

```rust
fn calculate_planet_distance_from_sun(period_seconds: f64) -> f64 {
    let mass_of_sun: f64 = 1.99e30;        // kg
    let gravity_constant: f64 = 6.67e-11;   // N⋅m²/kg²
    
    let numerator = gravity_constant * mass_of_sun * period_seconds.powi(2);
    let denominator = 4.0 * PI * PI;
    
    (numerator / denominator).powf(1.0/3.0)  // cube root
}
```

## Limitations

Kepler's laws assume:
- **Two-body system** — one object orbiting a much larger one
- **Newtonian gravity** — breaks down near black holes or at relativistic speeds
- **Circular/elliptical orbits** — not influenced by other gravitational bodies

For most planetary systems, including exoplanets, these assumptions hold true.

## License

MIT License — see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

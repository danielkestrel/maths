use std::f64::consts::PI;

fn calculate_planet_distance_from_sun(period: f64) -> f64 {
    
    let mass_of_sun: f64 = 1.99e30;
    let gravity_constant: f64 = 6.67e-11;
    let period_sqr = period * period;
    
    let numerator = gravity_constant * mass_of_sun * period_sqr;
    let denominator = 4.0 * PI * PI;
    
    let n = 1.0/3.0;
    
    let distance = (numerator / denominator).powf(n);
    
    distance
}

fn main() {
    let period_seconds = 365.25 * 24.0 * 60.0 * 60.0;
    let distance = calculate_planet_distance_from_sun(period_seconds);
    println!("The distance between the sun and the earth is {}", distance);
}

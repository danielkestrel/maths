use std::f64::consts::PI;

fn calculate_planet_distance_from_sun(period_seconds: f64) -> f64 {
    
    let mass_of_sun: f64 = 1.99e30;
    let gravity_constant: f64 = 6.67e-11;
    
    let numerator = gravity_constant * mass_of_sun * period_seconds.powi(2);
    let denominator = 4.0 * PI * PI;
    
    let distance = (numerator / denominator).powf(1.0/3.0);
    
    distance
}

fn main() {
    let period_seconds = 365.25 * 24.0 * 60.0 * 60.0;
    let distance = calculate_planet_distance_from_sun(period_seconds);
    println!("The distance between the sun and the earth is {}", distance);
}

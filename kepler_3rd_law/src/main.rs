use std::env;
use std::f64::consts::PI;
use std::process;

fn calculate_planet_distance_from_sun(period_seconds: f64) -> f64 {
    
    let mass_of_sun: f64 = 1.99e30;
    let gravity_constant: f64 = 6.67e-11;
    
    let numerator = gravity_constant * mass_of_sun * period_seconds.powi(2);
    let denominator = 4.0 * PI * PI;
    
    let distance = (numerator / denominator).powf(1.0/3.0);
    
    distance
}

fn print_usage() {
    eprintln!("Usage: planet_distance <period_in_days>");
    eprintln!("       planet_distance <period> <unit>");
    eprintln!();
    eprintln!("Units: days, hours, seconds");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  planet_distance 365.25");
    eprintln!("  planet_distance 365.25 days");
    eprintln!("  planet_distance 8760 hours");
}

fn main() {
    let args: Vec<String> = env::args().collect();
   
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    } 
    
    
    let period_value: f64 = match args[1].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Invalid number '{}'", args[1]);
            print_usage();
            process::exit(1);
        }
    };
    
    let unit = if args.len() >= 3 {
        args[2].to_lowercase()
    } else {
        "days".to_string()
    };
    
    let period_seconds = match unit.as_str() {
        "days" | "day" | "d" => period_value * 24.0 * 60.0 * 60.0,
        "hours" | "hour" | "h" => period_value * 60.0 * 60.0,
        "seconds" | "second" | "s" => period_value,
        _ => {
            eprintln!("Error: Unknown unit '{}'", unit);
            print_usage();
            process::exit(1);
        }
    };
    
    let distance = calculate_planet_distance_from_sun(period_seconds);
    let distance_km = distance / 1000.0;
    let au = distance / 1.496e11; // convert to astronomical units
    
    println!("Orbital period: {} {}", period_value, unit);
    println!("Distance from sun: {:.3e} meters", distance);
    println!("                   {:.3e} km", distance_km);
    println!("                   {:.3} Astronomical Unit (AU)", au);
}    

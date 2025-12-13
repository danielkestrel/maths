use average_speed::ave_speed::average_speed;

fn main() {
    // Float speeds
        let avg5 = average_speed(
            Speed::Float(30.5),
            Speed::Float(60.5),
        );
        println!("Floats: 30.5 and 60.5 -> {:?}", avg5);
        
        // Integer speeds
        let avg6 = average_speed(
            Speed::Integer(30),
            Speed::Integer(60),
        );
        println!("Integers: 30 and 60 -> {:?}", avg6);
        
        // Mixed!
        let avg7 = average_speed(
            Speed::Integer(25),
            Speed::Float(75.0),
        );
        println!("Mixed: 25 (int) and 75.0 (float) -> {:?}", avg7);
}
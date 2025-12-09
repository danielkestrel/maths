use average_speed::ave_speed::average_speed;
fn main() {

    let speed1 = 30.0;
    let speed2 = 60.0;

    println!("The average speed of {} and {} is: ", speed1, speed2);
    println!("{}", average_speed(speed1, speed2));
}
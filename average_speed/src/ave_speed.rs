//let's solve this together
//A driver sets out on a journey. For the first half of the distance, she drives at the
// leisurely pace of 30 mi/h; during the second half she drives 60 mi/h. What is her
// average speed on this trip?
pub fn average_speed(speed1: f64, speed2: f64) -> f64 {
    let total_distance = 2.0;

    let distance1 = total_distance / 2.0;
    let distance2 = total_distance / 2.0;

    let time1 = distance1 / speed1;
    let time2 = distance2 / speed2;

    //since dis

    let total_time = time1 + time2;

    total_distance/total_time
}
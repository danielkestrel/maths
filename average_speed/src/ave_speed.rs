//let's solve this together
//A driver sets out on a journey. For the first half of the distance, she drives at the
// leisurely pace of 30 mi/h; during the second half she drives 60 mi/h. What is her
// average speed on this trip?
use core::prelude::v1::derive;

#[derive(Debug, PartialEq)]
pub enum Speed{
    Float(f64),
    Integer(i64),   
}

pub fn average_speed(speed1: Speed, speed2: Speed) -> Speed {
    let (s1, s2) = match (speed1, speed2) {
        (Speed::Float(a), Speed::Float(b)) => (a, b),
        (Speed::Integer(a), Speed::Integer(b)) => (a as f64, b as f64),
        (Speed::Float(a), Speed::Integer(b)) => (a, b as f64),
        (Speed::Integer(a), Speed::Float(b)) => (a as f64, b),
    };
          
    // Calculate
    let half_distance = 1.0;
    let time1 = half_distance / s1;
    let time2 = half_distance / s2;
    let result = 2.0 / (time1 + time2);
          
    // Return as float (more accurate)
    Speed::Float(result)
}
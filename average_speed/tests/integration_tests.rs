use average_speed::ave_speed::average_speed;

#[test]
fn test_og_problem() {
    let result = average_speed(30.0, 60.0);
    
    assert_eq!(result, 40.0);
}

#[test]
fn test_equal_speed(){
    let result = average_speed(60.0, 60.0);
    
    assert_eq!(result, 60.0);
}

#[test]
fn test_symmetric_property(){
    let result1 = average_speed(30.0, 60.0);
    let result2 = average_speed(60.0, 30.0);
    
    assert_eq!(result1, result2);
}
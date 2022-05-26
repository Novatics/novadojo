use super::*;

#[test]
fn it_works() {
    let v1 = vec!["blue".to_string()];
    let v2 =  vec!["red".to_string()];
    let result = evaluate(v1, v2);
    assert_eq!(result, [0, 0]);
}

#[test]
fn it_work2() {
    let v1 = vec!["blue".to_string(),];
    let v2 =  vec!["blue".to_string(),];
    let result = evaluate(v1, v2);
    assert_eq!(result, [1, 0]);
}

#[test]
fn it_work3() {
    let v1 = vec!["red".to_string(),"yellow".to_string()];
    let v2 =  vec!["blue".to_string(),"red".to_string()];
    let result = evaluate(v1, v2);
    assert_eq!(result, [0, 1]);
}

#[test]
fn check_with_two() {
    let v1 = vec!["blue".to_string(),"blue".to_string()];
    let v2 =  vec!["blue".to_string()];
    let result = evaluate(v1, v2);
    assert_eq!(result, [2, 0]);
}
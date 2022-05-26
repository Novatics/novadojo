use super::*;
#[test]
fn it_works() {
    let v1 = ["blue"];
    let v2 =  ["red"];
    let result = evaluate(v1, v2);
    assert_eq!(result, [0, 0]);
}
#[test]
fn it_work2() {
    let v1 = ["blue"];
    let v2 =  ["blue"];
    let result = evaluate(v1, v2);
    assert_eq!(result, [1, 0]);
}
#[test]
fn it_work3() {
    let v1 = ["red","yellow"];
    let v2 =  ["blue","red"];
    let result = evaluate(v1, v2);
    assert_eq!(result, [0, 1]);
}
#[test]
fn check_with_two() {
    let v1 = ["blue","blue"];
    let v2 =  ["blue","red"];
    let result = evaluate(v1, v2);
    assert_eq!(result, [1, 0]);
}
#[test]
fn check_with_two_reversed() {
    let v1 = ["blue","red"];
    let v2 =  ["blue","blue"];
    let result = evaluate(v1, v2);
    assert_eq!(result, [1, 1]);
}


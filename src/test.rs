use super::*;

#[test]
// fn it_works() {
//     let result = sum(2, 2);
//     assert_eq!(result, 4);
// }

fn it_works() {
    let array1 = vec!["azul", "verde"];
    let array2 = vec!["vermelho", "verde"];

    let result = evaluate(array1, array2);
    assert_eq!(result, [1, 0]);
}

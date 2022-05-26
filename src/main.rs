#[cfg(test)]
mod test;

fn main() {
  let array1 = vec!["azul", "verde"];
  let array2 = vec!["vermelho", "verde"];

  let result = evaluate(array1, array2);
  println!("{result:?}");
}

fn evaluate(array1: Vec<&str>, array2: Vec<&str>) -> Vec<i32> {
  let mut well_placed = 0;
  let mut miss_placed = 0;
  for (pos,elem) in array1.iter().enumerate() {
    for (pos2,elem2) in array2.iter().enumerate() {
      if elem == elem2 {
         if pos2 == pos {
            well_placed = well_placed + 1;
         }
         else {
            miss_placed = miss_placed + 1;
         }
      }
    }
  }
  return vec![well_placed,miss_placed]
}

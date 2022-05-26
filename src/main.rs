#[cfg(test)]
mod test;
fn main() {
  let s: &[&str] = ["blue"];
  let g: &[&str] = ["blue"];
  evaluate(s, g);
}
fn evaluate(secret: &[&str], guess: &[&str]) -> Vec<u8> {
  //iterar
  let mut count_match: u8 = 0;
  let mut count_error: u8 = 0;

  for (i, x) in guess.iter().enumerate() {
    // count_match valor no lugar certo
    if secret[i] == *x {
      count_match += 1;
    }
    else {
      if secret.contains(&x) {
        count_error += 1;
      }
    }
  }

  return vec![count_match, count_error];
}
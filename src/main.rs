#[cfg(test)]
mod test;

fn main() {
  let s: Vec<String> = vec!["blue".to_string()];
  let g: Vec<String> = vec!["blue".to_string()];
  evaluate(s, g);
}

fn evaluate(secret: Vec<String>, guess: Vec<String>) -> Vec<u8> {
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

    // if (secret.contains(&x)) {
    //   count_match += 1;
    // }
  }
  println!("Secret: {:?}", secret);
  // println!("Guess: {:?}", guess);

  return vec![count_match, count_error];
}
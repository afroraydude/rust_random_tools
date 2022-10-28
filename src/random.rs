use std::io;

pub(crate) fn random_from_list() {
  // create a list from user input
  let mut list: Vec<String> = Vec::new();


  println!("Enter each list item on a line, or press enter to finish");
  loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    if input == "" {
      break;
    }
    list.push(input.to_string());
  }

  // get a random number
  let random_number = rand::random::<usize>() % list.len();

  // print the random item
  println!("The random item is: {}", list[random_number]);
}
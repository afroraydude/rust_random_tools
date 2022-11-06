use std::io;

pub(crate) fn random_from_list() {
    // create a list from user input
    let mut list: Vec<String> = Vec::new();
    loop {
        if list.len() == 0 {
            println!("Enter each list item on a line, or press enter to finish");
            loop {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let input = input.trim();
                if input == "" {
                    break;
                }
                list.push(input.to_string());
            }
        } else {
            println!("Picking a new random item from the list");
        }

        // get a random number
        let random_number = rand::random::<usize>() % list.len();

        // print the random item
        println!("The random item is: {}", list[random_number]);

        // ask if they want to continue
        println!("Would you like to continue? (y/r/n)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "n" {
            break;
        } else if input == "r" {
            list.clear();
        }
    }
}

pub fn magic_ball() {
    println!("Welcome to the magic 8 ball!");
    println!("What is your question?");
    let mut question = String::new();
    io::stdin()
        .read_line(&mut question)
        .expect("Failed to read line");
    let question = question.trim();
    let answers = vec![
        "It is certain",
        "It is decidedly so",
        "Without a doubt",
        "Yes definitely",
        "You may rely on it",
        "As I see it, yes",
        "Most likely",
        "Outlook good",
        "Yes",
        "Signs point to yes",
        "Reply hazy try again",
        "Ask again later",
        "Better not tell you now",
        "Cannot predict now",
        "Concentrate and ask again",
        "Don't count on it",
        "My reply is no",
        "My sources say no",
        "Outlook not so good",
        "Very doubtful",
    ];
    let random_number = rand::random::<usize>() % answers.len();
    println!("{}", answers[random_number]);
}

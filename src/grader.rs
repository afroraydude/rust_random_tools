use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct GradeItem {
    pub name: String,
    pub score: u32,
    pub total: u32,
}

impl GradeItem {
    fn new(name: String, score: u32, total: u32) -> GradeItem {
        GradeItem {
            name,
            score,
            total,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GradeCategory {
    pub name: String,
    pub weight: f32,
    pub grade: f32,
    pub grade_items: Vec<GradeItem>,
}

impl GradeCategory {
    fn new(name: String, weight: f32) -> GradeCategory {
        GradeCategory {
            name,
            weight,
            grade: 0.0,
            grade_items: Vec::new(),
        }
    }

    fn add_grade_item(&mut self, grade_item: GradeItem) {
        self.grade_items.push(grade_item);
    }

    fn calculate_grade(&mut self) {
        let mut total_score: u32 = 0;
        let mut total_total: u32 = 0;

        for grade_item in &self.grade_items {
            total_score += grade_item.score;
            total_total += grade_item.total;
        }

        self.grade = total_score as f32 / total_total as f32;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Grade {
    pub name: String,
    pub categories: Vec<GradeCategory>,
    pub final_grade: f32,
    pub letter_grade: String,
}

impl Grade {
    fn new(name: String) -> Grade {
        Grade {
            name: name,
            categories: Vec::new(),
            final_grade: 0.0,
            letter_grade: String::new(),
        }
    }

    fn add_category(&mut self, category: GradeCategory) {
        self.categories.push(category);
    }

    fn calculate_grade(&mut self) {
        let mut total_grade: f32 = 0.0;

        for category in &mut self.categories {
            category.calculate_grade();
            total_grade += category.grade * category.weight;
        }

        self.final_grade = total_grade;
        self.letter_grade = self.get_letter_grade();
    }

    fn get_letter_grade(&self) -> String {
        if self.final_grade >= 90.0 {
            String::from("A")
        } else if self.final_grade >= 80.0 {
            String::from("B")
        } else if self.final_grade >= 70.0 {
            String::from("C")
        } else if self.final_grade >= 60.0 {
            String::from("D")
        } else {
            String::from("F")
        }
    }

    fn save(&self, filename: &str) {
        let json = serde_json::to_string(&self).unwrap_or_else(|_| {
            println!("Failed to serialize grade");
            return;
        });
        std::fs::write(filename, json).unwrap_or_else(|_| {
            println!("Failed to write to file");
        });
    }

    fn load(filename: &str) -> Grade {
        let json = std::fs::read_to_string(filename).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}

pub fn grader() {
    println!("Welcome to the grade calculator!");
    /*
    Menu: 
    1. Load grade file
    2. Create new grade file
    */

    println!("Please choose an option from the menu:\n1. Load grade file\n2. Create new grade file\n");
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).unwrap();
    let option: u32 = option.trim().parse().unwrap();

    let mut grade: Grade;

    match option {
        1 => {
            println!("Please enter the filename of the grade file you would like to load:");
            let mut filename = String::new();
            std::io::stdin().read_line(&mut filename).unwrap();
            let filename = filename.trim();
            grade = Grade::load(filename);
        },
        2 => {
            println!("Please enter the name of the grade file you would like to create:");
            let mut filename = String::new();
            std::io::stdin().read_line(&mut filename).unwrap();
            let filename = filename.trim();
            grade = Grade::new(String::from(filename));
        },
        _ => {
            println!("Invalid option. Exiting...");
            return;
        }
    }

    loop {
        print_menu();
        let mut option = String::new();
        std::io::stdin().read_line(&mut option).unwrap();
        let option: u32 = option.trim().parse().unwrap();

        match option {
            1 => {
                add_category(&mut grade);
            }
            2 => {
                add_grade_item(&mut grade);
            }
            3 => {
                grade.calculate_grade();
                println!("Final grade: {}%", grade.final_grade);
            }
            4 => {
                println!("Please enter the filename of the grade file you would like to save:");
                let mut filename = String::new();
                std::io::stdin().read_line(&mut filename).unwrap_or_else(|err| {
                    println!("Error: {}", err);
                    0
                });
                let filename = filename.trim();
                grade.save(filename);
            }
            5 => {
                println!("Exiting...");
                return;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}

fn add_grade_item(grade: &mut Grade) {
    println!("Please enter the name of the category you would like to add a grade item to:");
    let mut category_name = String::new();
    std::io::stdin().read_line(&mut category_name).unwrap();
    let category_name = category_name.trim();

    let mut category_index: Option<usize> = None;

    for (index, category) in grade.categories.iter().enumerate() {
        if category.name == category_name {
            category_index = Some(index);
            break;
        }
    }

    if category_index.is_none() {
        println!("Category not found. Please try again.");
        return;
    }

    println!("Please enter the name of the grade item:");
    let mut grade_item_name = String::new();
    std::io::stdin().read_line(&mut grade_item_name).unwrap();
    let grade_item_name = grade_item_name.trim();

    println!("Please enter the score of the grade item:");
    let mut grade_item_score = String::new();
    std::io::stdin().read_line(&mut grade_item_score).unwrap();
    let grade_item_score: u32 = grade_item_score.trim().parse().unwrap();

    println!("Please enter the total score of the grade item:");
    let mut grade_item_total = String::new();
    std::io::stdin().read_line(&mut grade_item_total).unwrap();
    let grade_item_total: u32 = grade_item_total.trim().parse().unwrap();

    let grade_item = GradeItem::new(String::from(grade_item_name), grade_item_score, grade_item_total);
    grade.categories[category_index.unwrap()].add_grade_item(grade_item);
    println!("Grade item added successfully!");
}

fn add_category(grade: &mut Grade) {
    println!("Please enter the name of the category you would like to add:");
    let mut category_name = String::new();
    std::io::stdin().read_line(&mut category_name).unwrap();
    let category_name = category_name.trim();

    println!("Please enter the weight of the category:");
    let mut category_weight = String::new();
    std::io::stdin().read_line(&mut category_weight).unwrap();
    let category_weight: f32 = category_weight.trim().parse().unwrap();

    let category = GradeCategory::new(String::from(category_name), category_weight);
    grade.add_category(category);
    println!("Category added successfully!");
}

fn print_menu() {
    /*
    Menu:

    1. Add category
    2. Add grade item to category
    3. Calculate grade
    4. Save grade file
    5. Exit
    */

    println!("Please choose an option from the menu:\n1. Add category\n2. Add grade item to category\n3. Calculate grade\n4. Save grade file\n5. Exit\n");
}
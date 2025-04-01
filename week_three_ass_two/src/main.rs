use std::io;

fn main() { 
    /*
     * A rust program that:
     * - Ask a user how many students are in the class (min 10)
     * - For each user, prompt them to give their name and score
     * - For loop to process each students score
     * - Use control flow to assign grade based on the score
     * - - 70-100 - A
     * - - 60-69 - B
     * - - 55-59 - C
     * - - 45-54 - D
     * - - 40-45 - E
     * - - 0-39 - F
     */
    println!("How many users are in the class:");
    let mut num_of_students = String::new();
    let _ = io::stdin().read_line(&mut num_of_students).expect("Please enter a valid value");
    let num_of_students:u32 = num_of_students.trim().parse::<u32>().expect("Please enter a correct number of student");
    let mut count:u32 = 0;
    let mut list_of_student_name:Vec<String> = vec![];
    let mut list_of_student_score:Vec<f64> = vec![];
    while count < num_of_students {
        println!("Enter the number {} student's name:", count + 1);
        let mut student_name = String::new();
        let _ = io::stdin().read_line(&mut student_name).expect("Please enter a valid name");
        list_of_student_name.push(student_name);

        println!("Enter the number {} student's score:", count + 1);
        let mut student_score = String::new();
        let _ = io::stdin().read_line(&mut student_score).expect("Please enter a valid value");
        let student_score:f64 = student_score.trim().parse::<f64>().expect("Please enter a correct score");
        list_of_student_score.push(student_score);
        count += 1;

    }
    let students = create_students(list_of_student_name, list_of_student_score);

    for student in students {
        println!("{} scored {}", student.name.trim(), process_grade(student.score));
    }


}
struct Student {
    name: String,
    score: f64
}

fn create_students(list_of_student_name:Vec<String>, list_of_student_score:Vec<f64>) -> Vec<Student>{
    let mut list_of_student:Vec<Student> = vec![];
    let mut count = 0;
    while count < list_of_student_name.len() {
        list_of_student.push(Student { name: list_of_student_name[count].clone(), score: list_of_student_score[count] });
        count += 1;
    }
    return list_of_student;
}
fn process_grade(score: f64) -> String{
    if score > 100.0 {
        "Too much".to_string()
    } else if score >= 70.0 {
        "A".to_string()
    } else if score < 70.0 && score >= 60.0 {
        "B".to_string()
    } else if score < 60.0 && score >= 55.0  {
        "C".to_string()
    } else if score < 55.0 && score >= 45.0 {
        "D".to_string()
    } else if score < 45.0 && score >= 40.0 {
        "E".to_string()
    } else {
        "F".to_string()
    }
}

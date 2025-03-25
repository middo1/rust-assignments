fn main() {
    /*
     * A rust program that
     * stores student Record - at least 10
     * Calculate the total and average
     * Determine the highest scoring student
     * Counts how many students failed - Failure starts around < 50
     * Sorts the Student by pass and fail
     * Deadline Monday 4:00pm
     */
    let student_names: [&str; 10] = ["Adam", "Enoch", "Noah", "Abraham", "Issac", "Ishmael", "Moses", "Elijah", "Elisha", "Yeshua"];
    let student_scores: [u32; 10] = [57, 42, 67, 97, 54, 24, 65, 39, 81, 99];

    let student_data = create_students(student_names, student_scores);
    let mut failed: u32 = 0;
    let (highest_name, highest) = highest_score(&student_data);
    println!("The total score is {}", total_score(&student_data));
    println!("The Average score is {}", average(&student_data));
    println!("{} scored the highest score of {}", highest_name, highest);
    for student in student_data.iter() {
        if grade(student) {
            println!("{} failed", student.name);
            failed += 1;
        } else {
            println!("{} passed",student.name);
        }
    }
    println!("The number of student that failed is {}", failed);

}
struct Student {
    name: String,
    score: u32
}

fn create_students(student_names: [&str;10], student_scores: [u32;10]) -> Vec<Student> {
    let mut i = 0;
    let mut list_of_students = vec!();
    while i < student_names.len() {
        list_of_students.push(Student{
            name: student_names[i].to_string().clone(),
            score: student_scores[i]
        });
        i += 1;
    }
    list_of_students
}

fn total_score(list_of_students: &Vec<Student>) -> u32{
    let mut total: u32 = 0;
    for student in list_of_students.iter() {
        total += student.score
    }
    return total;
}

fn average(list_of_students: &Vec<Student>) -> f64 {
    total_score(list_of_students) as f64 / list_of_students.len() as f64
}

fn highest_score(list_of_students: &Vec<Student>) -> (&str, u32) {
    let mut highest: u32 = 0;
    let mut highest_name: &str = "";
    for student in list_of_students.iter() {
        if student.score > highest {
            highest = student.score;
            highest_name = &student.name;
        }
    }
    (highest_name, highest)
}

// fn failures(list_of_students: &[Student]) -> u32 {
//     let mut failed: u32 = 0;
//     for student in list_of_students.iter() {
//         if student.score <= 50 {
//             failed += 1;
//         }
//     }
//     failed
// }

fn grade(student: &Student) -> bool {
    if student.score <= 50 {
        return true
    }
    return false
}
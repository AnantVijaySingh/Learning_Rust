
#[derive(Debug)]
struct StudentBirthday {
    name: String,
    birthday: u32
}

fn main() {
    let birthday_list = generate_list_of_student_birthdays();

    for student in &birthday_list {

        let (matched, name_if_match) = compare_birthdays(&birthday_list, &student);

        if matched {
            println!("{} has the same birthday as {}", student.name, name_if_match);
        }
    }
}

fn compare_birthdays (list: &[StudentBirthday;5], student: &StudentBirthday) -> (bool, String) {
    for students_in_list in list {

        if student.birthday == students_in_list.birthday && student.name != students_in_list.name{
            let name = students_in_list.name.clone();
            return (true, name)
        }
    }
    (false, String::from("none"))
}


fn generate_list_of_student_birthdays() -> [StudentBirthday; 5] {
    let list = [
        StudentBirthday{
        name: String::from("Harry"),
        birthday: 31071980
        },
        StudentBirthday{
            name: String::from("Hermione"),
            birthday: 19091979
        },
        StudentBirthday{
            name: String::from("Ron"),
            birthday: 01031980
        },
        StudentBirthday{
            name: String::from("Fred"),
            birthday: 01041978
        },
        StudentBirthday{
            name: String::from("George"),
            birthday: 01041978
        }
    ];

    list
}
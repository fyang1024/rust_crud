use crate::student_store::StudentStore;

mod class;
mod class_store;
mod club;
mod club_store;
mod grade;
mod lesson;
mod lesson_store;
mod student;
mod student_store;

fn main() {
    let mut class_store: class_store::ClassStore = class_store::ClassStore::new();
    let new_class = class_store.create_class(grade::Grade::EIGHT, 1);
    println!("created class ==> {:?}\n", new_class);

    let mut student_store: student_store::StudentStore = student_store::StudentStore::new();

    let student = student_store.create_student(String::from("Alice"), new_class);
    println!("created student ==> {:?}\n", student);

    let found_class = class_store.find_class(&grade::Grade::EIGHT, &1);
    println!("found class ==> {:?}\n", found_class);

    let found_student_mut = student_store.find_student_mut(&0);
    println!("found student mutable ==> {:?}\n", found_student_mut);

    println!("updating student 0 name from Alice to Bob");
    found_student_mut
        .unwrap()
        .set_full_name(String::from("Bob"));

    let found_student = student_store.find_student(&0);
    println!("found student ==> {:?}\n", found_student);

    println!("updating student 0 name from Bob back to Alice");
    student_store.update_student_full_name(&0, String::from("Alice"));

    let found_student = student_store.find_student(&0);
    println!("found student ==> {:?}\n", found_student);

    let mut club_store = club_store::ClubStore::new();

    let new_club =
        club_store.create_club(String::from("Cryptopia"), String::from("Crypto Club"), 100);
    println!("new club ==> {:?}\n", new_club);

    let found_club = club_store.find_club(&String::from("Cryptopia"));
    println!("found club ==> {:?}\n", found_club);

    let found_club_mut = club_store
        .find_club_mut(&String::from("Cryptopia"))
        .unwrap();
    let found_student_mut = student_store.find_student_mut(&0).unwrap();
    found_student_mut.join_club(found_club_mut);

    println!("student after joining club ==> {:?}\n", found_student_mut);
    println!("club after student joining ==> {:?}\n", found_club_mut);

    let mut lesson_store = lesson_store::LessonStore::new();
    let new_lesson = lesson_store.create_lesson(
        String::from("Rust"),
        String::from("Rust Rocks"),
        vec![
            grade::Grade::TEN,
            grade::Grade::ELEVEN,
            grade::Grade::TWELVE,
        ],
    );
    println!("new lesson ==> {:?}\n", new_lesson);

    lesson_store
        .update_lesson_description(&String::from("Rust"), String::from("Rust still rocks!"));

    let lesson_found_mut = lesson_store.find_lesson_mut(&String::from("Rust")).unwrap();
    println!("lesson found ==> {:?}\n", lesson_found_mut);
    lesson_found_mut.set_grades(vec![
        grade::Grade::NINE,
        grade::Grade::TEN,
        grade::Grade::ELEVEN,
        grade::Grade::TWELVE,
    ]);
    let lesson_found = lesson_store.find_lesson(&String::from("Rust")).unwrap();
    println!("updated lesson ==> {:?}\n", lesson_found);

    let found_class_mut = class_store
        .find_class_mut(&grade::Grade::EIGHT, &1)
        .unwrap();
    found_class_mut.remove_student(&0);
    println!("class after removing student ==> {:?}\n", found_class_mut);

    let found_student_mut = student_store.find_student_mut(&0).unwrap();
    let found_club_mut = club_store
        .find_club_mut(&String::from("Cryptopia"))
        .unwrap();
    found_student_mut.leave_club(found_club_mut);

    println!("student after leaving club ==> {:?}\n", found_student_mut);
    println!("club after student leaving ==> {:?}\n", found_club_mut);

    student_store.delete_student(0);
    println!("deleted student 0");
}

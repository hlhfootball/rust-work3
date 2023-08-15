use std::{collections::HashMap};

#[derive(Debug)]
pub struct Student {
    id: u32,
    name: String,
    class_id: u32,
}

pub struct Club {
    id: u32,
    name: String,
    member_ids: Vec<u32>,
}

pub struct Class {
    id: u32,
    name: String,
    student_ids: Vec<u32>,
}

pub struct Course {
    id: u32,
    name: String,
    class_id: u32,
}

pub struct StudentManagementSystem {
    students: HashMap<u32, Student>,
    clubs: HashMap<u32, Club>,
    classes: HashMap<u32, Class>,
    courses: HashMap<u32, Course>,
}

impl StudentManagementSystem {
    // 创建一个新的学生管理系统
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
            clubs: HashMap::new(),
            classes: HashMap::new(),
            courses: HashMap::new(),
        }
    }

    // 学生的 CRUD 操作
    pub fn add_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }

    pub fn delete_student(&mut self, student_id: u32) {
        self.students.remove(&student_id);
    }
}

fn main() {
    let mut sms = StudentManagementSystem::new();

    sms.add_student(Student {
        id: 1,
        name: "Alice".to_string(),
        class_id: 301,
    });

    sms.add_student(Student {
        id: 2,
        name: "Bob".to_string(),
        class_id: 205,
    });

    println!("{:?}", sms.students);
}

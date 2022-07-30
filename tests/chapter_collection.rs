
#[cfg(test)]
mod test{

    #[derive(Debug,Default)]
    struct Student{
        pub name: Option<String>,
        pub age: Option<i32>,
        pub gender: Option<bool>,
    }

    /// 在循环中使用 Vector (动态数组)
    #[test]
    fn test_vec_mut(){
        let mut students = Vec::<Student>::new();
        let mut student_vec = Vec::<Student>::new();
        students.push(Student{
            name: Some("小明".to_string()),
            age: Some(18),
            gender: Some(true),
        });
        students.push(Student{
            name: Some("小红".to_string()),
            age: Some(20),
            gender: Some(false),
        });
        /// 这个循环要 消费掉 每个遍历到的元素
        for mut student in students{
            student.name = Some("韩信".to_string());
            student.age = Some(25);
            student.gender = Some(true);
            student_vec.push(student)
        };
        println!("student_vec = {:?}",student_vec);
    }

    /// 只有对象的实例是可变的，才能定义它的引用是可变的。(只能获取可变对象的可变引用，不能获取不可变对象的可变引用)
    #[test]
    fn test_borrow_and_borrow_mut(){
        /// 只有对象的实例是可变的，才能定义它的引用是可变的。(只能获取可变对象的可变引用，不能获取不可变对象的可变引用)
        //let student = Student::default();
        //let student_borrow_mut = &mut student;// Error: 只有 Student 的实例 student 是 mut(可变的)，才能定义它的引用是可变的

        let mut student = Student::default();
        let student_mut = &mut student;

    }
}
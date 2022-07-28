
#[cfg(test)]
mod test{

    #[derive(Debug)]
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

}
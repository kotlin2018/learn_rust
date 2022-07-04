
#[cfg(test)]
mod test{
    use std::collections::HashMap;
    use tracing_subscriber::fmt::try_init;

    // Box智能指针
    // Copy trait 是值语义的复制，rust编译器自动为基本数据类型实现该 trait
    // Clone trait 是引用语义的深复制，引用类型调用 clone 方法，以实现深度克隆。
    // 能 Copy 的就是值语义，不能的就是引用语义。
    #[test]
    fn test_box(){
        // 这是错误的写法，Box智能指针不能实现 Copy trait
        // #[derive(Debug,Copy,Clone)]
        // struct A {
        //     a: i32,
        //     b: Box<i32>, //Error，Box智能指针不能实现 Copy trait
        // }
        // let a = A{a:5,b:Box::new(6)};
        // println!("{:?}",a);

        // Box 唯一所有权智能指针能实现 Clone trait
        // #[derive(Debug,Clone)]
        // struct B {
        //     a: i32,
        //     b: Box<i32>,
        // }
        // let b = B{a: 3,b: Box::new(2)};
        // println!("{:?}",b);

        let mut map = HashMap::<String,i32>::new();
        map.insert("aa".to_string(),11);
        map.insert("bb".to_string(),22);
        assert_eq!(map.len(),2);
    }

    // 同一个HashMap存储不同的 Key Value 键值对
    #[test]
    fn test_hash_map(){
        /// 这种实现方式报错了
        // trait IHashMap{}
        // struct Go {}
        // impl IHashMap for Go{}
        //
        // trait IHashMap2{}
        // struct Rust{}
        // impl IHashMap2 for Rust{}
        //
        // let mut hash_map = HashMap::<Box<dyn IHashMap>,Box<dyn IHashMap2>>::new();
        // hash_map.insert(Box::new(Go{}),Box::new(Rust{}));
        // assert_eq!(hash_map.len(),1);
    }
}
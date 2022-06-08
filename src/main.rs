extern crate core;
#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::fmt::Formatter;
#[allow(unused)]
macro_rules! unit {
    ($($x:tt)*) => {
        ()
    };
}

macro_rules! cap {
    ($($key:expr),*) => ( (&[ $(unit!($key)),*]).len());
}

macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)*) => {
        {
            let _cap = cap!($($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                _map.insert($key,$value);
            )*
            _map
        }
    };
}

static sum: fn(i32,i32)->i32 = |i:i32,j:i32|->i32{
    sum(1,1);
    i+j
};



fn main() {
   // sum(1,3);
   // println!("Hello, world!");
}

#[cfg(test)]
#[allow(unused)]
pub mod test{
    use std::num::ParseIntError;
    use std::ptr::addr_of;
    use std::string::ParseError;
    use std::sync::{Arc, Mutex};
    use std::{fmt, mem, thread};
    use std::fmt::{Display, Formatter};
    use std::mem::swap;
    use std::thread::park;
    use super::*;

    // 所有权与类型系统，不同的类型拥有不同的行为 trait，例如:Copy(),Move()
    //
    // 资源最多只能被使用一次
    //
    // 移动语义(Move)  资源 -----消耗------>资源
    //
    // 在运行时，动态增长的类型是被分配到堆内存，例如:动态增长的字符串、数组，同时栈上保存了一个指向该堆内存的指针，
    // 如果该类型实现了 Copy 语义，则栈上会有两个指针来指向这同一块堆内存，相当于有两个指针来控制同一块堆内存，这是不安全的。
    // 因此 动态增长的类型只能实现 Move 语义。
    //
    // 在运行时，
    //
    //
    // 复制语义(Copy)  资源 -----复制------>资源
    //
    // 所有权语义下的内存管理方式:
    //
    // 1、默认存储数据，是存储在栈上。
    //
    // 2、利用栈来自动管理堆内存
    //
    // (当函数调用结束的时候，定义在函数内部的本地变量会被清空，放在栈上的，指向堆内存的指针会被清理，同时该指针指向的堆内存也会被清理)
    //
    // 函数栈调用结束时，栈上的指针指向的堆内存会先被析构，然后栈上指针才被清理，这样就保证了不会出现悬垂指针

    // 在同样的操作系统上 rust 中 借用(引用)的大下是固定的，因此在编译期借用类型能被编译通过，并且借用将被保存在 栈上

    // rust std 中所有类型都自动实现了 Debug trait，都可以使用 {:?} 输出, {:#?} 是Debug 美化输出

    // 如果类型 A 实现了 From<B>,则B类型实例调用 into() 方法就可以转换为类型 A
    // Rust 为所有实现 From 的自动实现了反方向的 Into


    // #[test]
    // fn test_ownership(){
    //
    // }

    // 使用 Arc 和 Mutex 在线程间安全共享数据
    #[test]
    fn list_foreach(){
        let v = Arc::new(Mutex::new(vec![1,2,3,4]));

        for i in 0..3{
            let cloned_v = v.clone();
            thread::spawn(move ||{  // cloned_v 所有权被 move 到了闭包内部
               cloned_v.lock().unwrap().push(i);
            });
            //println!("{:?}",cloned_v); //这里输出报错
        }
    }
    trait Currying{
        type ReturnType: Fn(i32) -> i32;
        fn add(self) ->Self::ReturnType;
    }

    struct States<'a>{
        a: &'a i32,
        b: &'a i32,
    }

    impl Currying for States<'static> {
        type ReturnType = Box<dyn Fn(i32) -> i32>;

        fn add(self) -> Self::ReturnType {
            Box::new(move|x|{
                x * self.a
            })
        }
    }

    #[test]
    fn test_01 () {
        let r_value = States{
            a: &100,
            b: &100,
    };
        let r1 = r_value.add();
        let r2 = r1(5);
        assert_eq!(500,r2)
    }

    // #[test]
    // fn test_02(){
    //     let result: Result<i32,ParseIntError> = try{// try 关键字目前还不能使用
    //         "1".parse::<i32>()?
    //         +"2".parse::<i32>()?
    //         +"3".parse::<i32>()?
    //     };
    //     assert_eq!(result,Ok(6));
    // }

    #[test]
    fn test_03(){
        let a = i8::MAX;
        println!("{}",a); // 输出 127 , i8 类型能表达的最大值是 127

        let a = 3.1 as i8; // as 类型转换
        let b = 100_i8 as i32;
        let c = 'a' as u8;
        println!("{},{},{}",a,b,c);
    }

    #[test]
    fn test_04(){
        let map = hashmap!{
            "a" => 1,
            "b" => 2,
            "c" => 3,
        };

        assert_eq!(map["a"],1);
    }

    // #[test]
    // fn test_05(){
    //     let v = vec![1u64,2,3,4,5,6];
    //     let val = v.iter().
    //         enumerate().filter(|&(idx,_)|idx %2 == 0)
    //         .map(|(idx,val)|val)
    //         .fold(0u64,|sum,acm|sum+acm);
    //     println!("{}",val);
    // }


    // struct SelfReferential<'a>{
    //     a: String,
    //     b: &'a String,
    // }
    //
    // #[test]
    // fn test_06(){
    //     let a = String::from("hello world");
    //     let _sr = SelfReferential{a,b:&a};
    // }
    // #[cfg(test)]
// pub mod test{
//     use super::*;
//     #[test]
//     fn test<T: Send + Sync +fmt::Display + 'static>(val: T){
//         thread::spawn(move || print!("{}",val));
//     }
// }

    // 闭包三部分: 参数、返回值、执行体
    #[test]
    fn test_07(){
        // 完整语法 (参数、返回值、执行体)
        let add = |a: i32,b: i32| ->i32 {
            a + b
        };
        assert_eq!(add(1,2),3)

    }

    // static sum: fn(i32,i32)->i32 = |i:i32,j:i32|->i32{
    //     sum(1,1);
    //     i+j
    // };
    // #[test]
    // fn test_08(){
    //     sum(1,3);
    // }

    struct Foo{
        bar: String,
    }

    impl Foo {
        fn print(&self){
            println!("{}",self.bar);
        }
        fn mut_print(&mut self){
            self.bar = "Hello".to_string();
            self.print();
            self.bar = "word".to_string();
        }
    }
    #[test]
    fn test_09(){
        let mut a = Foo{bar:"3".to_string()};
        a.mut_print();
    }

    #[derive(Debug)]
    pub struct Person{
        age: i32,
        name: String,
    }
    #[test]
    fn test_10(){
        let x = 5 + /* 90 + */ 5;
        println!("{:?}",x);

        let p = Person{age: 19,name: "小明".to_string()};
        println!("{:#?}",p); // Debug 美化输出
    }

    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f,"[")?;
            for (count,v) in vec.iter().enumerate(){
                if count !=0 {
                    write!(f,",")?;}
                write!(f,"{}",v)?;
            }
            write!(f,"]")
        }
    }

    #[test]
    fn test_11(){
        let v = List(vec![1,2,3]);
        println!("{}",v);
    }

    #[derive(Debug)]
    struct Any{
        content: String,
    }

    impl From<i32> for Any{
        fn from(u: i32) -> Self {
           Any{
               content: format!("{}",u)
           }
        }
    }

    #[test]
    fn test_12(){
        let a = Any::from(1);
        println!("{:?}",a);

        let b: Any = 2.into();
        println!("{:?}",b);

        let string = String::from("hello world"); // String 构造自身实例
        let str: String = string.into(); // 从自身实例转换成自身
        println!("{:?}",str);

    }

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }
    impl Display for City{
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 {'N'}else {'S'};
            let lon_c = if self.lon >= 0.0 {'E'}else {'W'};
            write!(f,"{}: {:.3}°{} {:.3}°{}",
                   self.name,self.lat.abs(),lat_c,self.lon.abs(),lon_c)
        }
    }

    #[test]
    fn test_13(){
        for city in [
            City{name:"Dublin",lat: 53.347778, lon: -6.259722 },
            City { name: "Oslo", lat: 59.95, lon: 10.75 },
            City { name: "Vancouver", lat: 49.25, lon: -123.1 },
        ].iter(){
            println!("{}", *city);
        }
    }

    // 字面量和运算符
    #[test]
    fn test_14(){
        // 整数相加
        println!("1 + 2 = {}",1_u32+2);
        // 整数相减
        println!("1 - 2 = {}",1_i32 - 2);
        // 短路求职的布尔运算
        println!("true and false is {}",true&&false);
        println!("true or false is {}",true || false);

    }

    fn reverse(pair: (i32,bool)) -> (bool,i32){
        let (integer,boolean) = pair;
        (boolean,integer)
    }

    #[derive(Debug)]
    struct Matrix(f32,f32,f32,f32);

    #[test]
    fn test_15(){
        let long_tuple = (1u8,2u16,3u32,4u64);
        println!("long tuple first value: {}",long_tuple.0);

        let pair = (1,true);
        println!("pair is {:?}",pair);
        println!("the reversed pair is {:?}",reverse(pair));

        let tuple = (1,"hello",4.5,true);
        let (a,b,c,d) = tuple;
        println!("{:?},{:?},{:?},{:?}",a,b,c,d);

        let matrix = Matrix(1.1,1.2,2.1,2.2);
        println!("{:?}",matrix);

    }

    // i32类型切片
    fn analyze_slice(slice: &[i32]){
        println!("first element of the slice: {}",slice[0]);
        println!("the slice has {} elements",slice.len());
    }
    #[test]
    fn test_16(){
        let xs = [1,2,3,4,5];
        // 下标从 0 开始
        println!("first element of the array: {}",xs[0]);
        println!("second element of the array: {}",xs[1]);
        println!("array size: {}", xs.len());
        // 数组是在栈中分配的
        println!("array occupies {} bytes",mem::size_of_val(&xs));
        // 数组可以自动借用成为 slice
        println!("borrow the whole array as a slice");
        analyze_slice(&xs);

        let ys = [0;500];
        // slice 可以指向数组的一部分
        println!("borrow a section of the array as a slice");
        analyze_slice(&ys[1..4]);

        // 越界的下标会引发致命错误(panic)
        println!("{}",xs[5]);

    }

    #[derive(Debug)]
    enum Number{
        Zero,
        One,
        Two,
    }

    #[derive(Debug)]
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    #[test]
    fn test_17(){
        println!("zero is {:?}",Number::Zero as i32);
        println!("one is {:?}",Number::One as i32);

        println!("roses are #{:?}",Color::Red as i32);
        println!("violets are #{:?}",Color::Blue as i32);

        println!("zero is {:?}",Number::Zero);
        println!("one is {:?}",Number::One);

        println!("roses are #{:?}",Color::Red);
        println!("violets are #{:?}",Color::Blue);
    }

    // 类型别名
    type NanoSecond = u64; // u64 的类型别名是 NanoSecond
    type Inch = u64;
    #[allow(non_camel_case_types)]
    type u64_t = u64;
    #[test]
    fn test_18(){
        let nano_seconds = 5 as NanoSecond;
        let inches = 2 as Inch;
        println!("{} nano_seconds + {} inches = {} unit?",
        nano_seconds,
        inches,
        nano_seconds + inches);
    }

    struct ToDrop;

    impl Drop for ToDrop {
        fn drop(&mut self) {
            print!("ToDrop 被析构了");
        }
    }
    #[test]
    fn test_19(){
        let x = ToDrop;
        println!("Mode a ToDrop!");
        // 先输出 Mode a ToDrop! ，再输出 ToDrop 被析构了
        // 说明 x 变量 在离开作用域之前被调用
    }

    #[test]
    fn test_20(){
        let s1 = "panama";
        let s2 = &String::from("banana");
    }

    #[test]
    fn test_21(){
        let mut n = 0;
        let mut count =  move||{
            n += 1;
            println!("Inner1: {}",n);
        };
        count(); // 这里输出 1
        println!("Outer2: {}",n); // 这里输出 0
        count(); // 这里输出 2 (这里为啥会输出 2 咧) 难道闭包能缓存内部变量的值？
        println!("Outer3: {}",n); // 这里输出 0
        count();
    }

    #[test]
    fn test_22(){
        let mut n = 0;
        let mut counter = ||{
            n += 1;
            println!("Inner1: {}",n);
        };
        counter();
        counter();
        println!("outer: {}",n);
    }

    #[test]
    fn test_23(){
        let mut n = 0;
        let mut count =  move||{
            n += 1;
            println!("Inner1: {}",n);
        };
        count(); // 这里输出 1
        println!("Outer2: {}",n); // 这里输出 0
        count(); // 这里输出 2 (这里为啥会输出 2 咧) 难道闭包能缓存内部变量的值？
        println!("Outer3: {}",n); // 这里输出 0
        count();
    }

    fn another()->i32{
        let x = 1;
        x
    }
    #[test]
    fn test_24(){
        let x = another();
        println!("{}",x);
    }

    // #[test]
    // fn test_25(){
    //     let mut data = vec![1,2,3];
    //     let x = &data[0];
    //     println!("{}",x);
    //     data.push(4);
    //     //println!("{:?}",data);
    //     let x = data[0];
    //     println!("{}",x);
    // }

    // rust 中函数无法捕获环境变量 (闭包可以)
    // fn counter(i: i32) -> fn(i32) ->i32{ // fn(i32) ->i32 是一个函数指针，函数可以用一个变量来绑定，因此函数也可以是指针类型
    //     fn inc(n: i32)-> i32 {
    //     n + i // 这里 inc 无法捕获 counter()函数的参数 i
    // }
    //     inc // 返回这个函数指针
    // }
    #[test]
    fn test_26(){
        // let f = counter(2); // 这段代码编译会报错
        // assert_eq!(3,f(1));
    }

    // 使用闭包捕获环境变量
    fn counter(i: i32) -> impl FnMut(i32)->i32{  // FnMut 是个 trait
        // println!("{}",n);
        println!("{}",i);
        move |n| n+i // 这是一个闭包，它的类型是 FnMut(i32)->i32
    }
    #[test]
    fn test_27(){
        let mut f = counter(2); //  i = 2
        // 这里 f 的类型是 FnMut(i32)->i32，因此 f(1) 执行的代码是这个闭包 |n| n+i
        assert_eq!(3,f(1));  // counter()内的闭包被调用 闭包的 入参 n = 1 ,则 n+i = 1+2
    }

    #[test]
    fn test_28(){
        let a = 1;
        let b = a + 1;
        let add_one_v2 = |x: u32|-> u32{x+1};
        let add_one_v3 = |x|{x + 1};
        let add_one_v4 = |x|x+1;
        let a = add_one_v2(5);
        let b = add_one_v3(5);
        let c = add_one_v4(5);
        println!("a = {},b = {}, c = {}",a,b,c);
    }

    #[test]
    fn test_29(){
        // 未捕获环境
        let c1 = ||println!("hello");
        c1();

        // 可修改环境变量
        let mut arr = [1,2,3];
        let mut c2 = |i|{
            arr[0] = i; // arr[0] 这个变量指向 指向(绑定) i 这个值
            println!("{:?}",arr);
        };
        c2(0);

        // 未修改环境变量
        let anwser = 42;
        let c3 = ||{
            println!("{}",anwser)
        };
        c3();
    }

    #[test]
    fn test_30(){
        let mut s = String::from("hello");
        // let ref1 = &s;
        // let ref2 = &ref1;
        // let ref3 = &ref2;
        // //let ss = String::from("goodbye");
        // let  s = String::from("goodbye");
        // println!("{}",ref3.to_uppercase());
        s = String::from("goodbye");
    }

    #[test]
    fn test_31(){
        let a = 1;   // a 是 i32 类型，默认实现了 Copy trait,并且 a 指向的值是存储在栈上，因此 a 绑定的值可以被修改，计算
        let b = a+1;
        println!("{}",a);

        // let c = Box::from(3); // c 指向的变量 3 存储在堆上，存储在堆上的只实现了 Move trait
        // let d = c;
        // println!("{}",c); // 这里报错
    }

    #[test]
    fn test_32(){
        // 逃逸闭包: 能被函数返回，不在函数调用过程中被销毁的闭包
        fn c_mut() -> impl FnMut(i32)->[i32;3]{ // 闭包是这个类型说明，闭包要修改环境变量
            // arr 是一个局部变量，它会随着函数的调用完毕而被消亡(drop)
            // 只有通过 move 关键字将 arr 的所有权转移到闭包内，再通过闭包将 arr 带出函数
            let mut arr = [0,1,2];
            move |i|{arr[0] = i;arr}
        }
        let i = 42;
        let mut arr_closure = c_mut();
        println!("{:?}",arr_closure(i));
        let mut result = arr_closure(i);
        println!("{:?}",result);
    }
}
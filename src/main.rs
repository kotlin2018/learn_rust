extern crate core;
#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::fmt::Formatter;
use std::thread::sleep;
use std::time;
use tokio::runtime::Builder;

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

#[derive(Debug)]
struct People{
    name: String,
    age: u32,
}

impl Drop for People{
    fn drop(&mut self) {
        println!("people is already drop")
    }
}

async fn nice(){
    sleep(time::Duration::from_secs(1))
}
fn main() {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(
        async{
            tokio::spawn(async{
                nice().await;
                println!("YesYes")
            });
            println!("HiHi");
            100
        }
    );
    print!("{}",v)
}

#[cfg(test)]
#[allow(unused)]
pub mod test{
    use std::num::ParseIntError;
    use std::ptr::addr_of;
    use std::string::ParseError;
    use std::sync::{Arc, Mutex};
    use std::{fmt, mem, thread, time};
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use std::fs::OpenOptions;
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
        // let a1 = 1;
        // let b1 = a1 + 1;
        let add_one_v2 = |x: u32|-> u32{x+1};
        let add_one_v3 = |x|{x + 1};
        let add_one_v4 = |x|x+1;
        let a = add_one_v2(5);
        let b = add_one_v3(5);
        let c = add_one_v4(5);
        println!("a = {:?},b = {:?}, c = {:?}",a,b,c);
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
        // let a = 1;   // a 是 i32 类型，默认实现了 Copy trait,并且 a 指向的值是存储在栈上，因此 a 绑定的值可以被修改，计算
        // let b = a+1;
        // println!("{}",a);

        let c = Box::from(3); // c 指向的变量 3 存储在堆上，存储在堆上的只实现了 Move trait
        let d = &c; // 这里只是将c的借用给了d，c的所有权还是在c自身
        println!("{}",c); // Ok

        // let c = Box::from(3); // c 指向的变量 3 存储在堆上，存储在堆上的只实现了 Move trait
        // let d = c; // 这里直接将c自身给了d，c的所有权已经转移到了d，c已经被消耗掉了
        // println!("{}",c); // Error
    }

    #[test]
    fn test_32(){
        // 逃逸闭包: 能被函数返回，不在函数调用过程中被销毁的闭包。

        // FnMut 用作逃逸闭包
        fn c_mut() -> impl FnMut(i32)->[i32;3]{ // 闭包是这个类型说明，闭包要修改环境变量
            // arr 是一个局部变量，它会随着函数的调用完毕而被消亡(drop)
            // arr 中的所有元素都是基本数据类型，都实现了Copy trait，因此 arr 也就实现了 Copy trait
            // move 只是将 arr 的一个副本移动到了闭包内，闭包则将 arr这个副本带出当前函数
            let mut arr = [0,1,2];
            move |i|{arr[0] = i;arr}
        }
        let i = 42;
        let mut arr_closure = c_mut();
        println!("{:?}",arr_closure(i));
        let mut result = arr_closure(i);
        println!("{:?}",result);

        // FnMut 不能用作逃逸闭包
        // fn c_mut2()-> impl for<'a> FnMut(&'a str)->String{
        //     // s 是动态可增长的字符串，它的值存储在堆上
        //     // 随着函数调用完毕，s 指向的 存储在堆上的值会被 drop,同时 s 也会被 drop
        //     // 但是此时 s 的指针又被 move 进了闭包，并被闭包带出了函数，此时 s就成了悬垂指针
        //     // 这 move 是可有可无的，s 没有实现 Copy，即使不使用 move，闭包捕获这个 s 也会自动转移 s 的所有权
        //
        //     let mut s = "hello".to_string();
        //     move |i|{s += i;s}
        // }
        // let i = "world";
        // let mut arr_closure = c_mut2(); // Error

        // 即使不使用闭包，也无法返回一个局部变量的引用，主要是为了防止出现悬垂指针。
    }

    #[test]
    fn test_33(){

        // rustc 不允许你去使用被闭包捕获的引用(借用)

        // [唯一不可变引用]: 被闭包捕获的引用
        let mut a = [1,2,3];
        let x = &mut a;

        // 结论: rustc 不允许你去使用被闭包捕获的引用(借用)
        // let mut c = ||{(*x)[0] = 0;};
        // let y = &x; //Error
        // c();
        // let z = &x; //OK
    }

    // 闭包实现 Copy/Clone 的两条规则
    // 1、如果环境变量实现了Copy，闭包如果以可变借用方式捕获环境变量，并对其进行修改，则闭包自身不会实现Copy
    // ( 如果闭包对环境变量产生影响，这个闭包自身就不能实现Copy
    // 如果这个闭包能实现Copy，相当于多个闭包来对环境变量进行修改，这违反Rust可变借用的规则

    // 2、如果环境变量自身是 Move 语义，则闭包内捕获环境变量的操作涉及修改环境或者消耗环境变量，则闭包自身不会实现Copy

    // 实现 Sync/Send 的三条简单规则
    // 1、如果所有捕获变量均实现了Sync，则闭包实现Sync
    // 2、如果环境变量都不是 [唯一不可变引用] 方式捕获的，并且都实现了 Sync，则闭包实现Send
    // 3、如果环境变量是以 [唯一不可变引用]、[可变引用]、Copy或Move所有权捕获的，那闭包实现Send
    fn foo<F: Fn() + Copy>(f: F){
        f()
    }

    #[test]
    fn test_34(){
        let s = "hello".to_owned();
        // 当前闭包捕获了环境变量 s，但是并未修改s，因此该闭包实现了 Fn trait，它是 Fn 类型，这种情况下，闭包自身也实现了Copy trait
        let f = || {
            println!("{}",s)
        };
        foo(f); //Ok

        // let ss = "hello".to_owned();
        // // move 会强制将 ss 转移到闭包中，相当于闭包消耗掉了环境变量 ss，此时闭包对环境变量产生了影响，因此该闭包不能实现Copy trait
        // // 如果 闭包实现了 Copy，则它会消耗两次环境变量，这不符合 rust 的规则。
        // let ff = move ||{
        //     println!("{}",b);
        // };
        // foo(ff);//Error
    }


    #[test]
    fn test_35(){  // 本地线程的使用
        let h1 = thread::spawn(||{
            println!("Delay par 1s");
            thread::sleep(time::Duration::from_millis(1))
        });

        let h2 = thread::spawn(||{
            println!("Delay per 1.5s");
            thread::sleep(time::Duration::from_millis(1));
        });
        h1.join(); // h1 加入线程池，因此可以被调度执行
        h2.join(); // h2 加入线程池，因此可以被调度执行
    }

    // 模式匹配
    struct Point{
        x: i32,
        y: i32,
    }

    #[test]
    fn test_36(){
        // let 声明模式匹配
        let (a,b) = (1,2);
        assert_eq!(1,a); //Ok
        assert_eq!(2,b); //Ok

        let Point{x,y} = Point{x: 3,y: 4};
        assert_eq!(3,x); //Ok
        assert_eq!(y,4); //Ok

        // 函数与闭包模式匹配
        fn sum(x: String,ref y: String) -> String{
            x + y
        }
        let s = sum("1".to_string(),"2".to_owned());
        assert_eq!(s,"12".to_owned()); //Ok

        // ref 模式匹配
        let a = 42;
        let ref b = a; // 此时 b 是 &i32 类型，等价于 b = &a  ( ref 把变量 a 的借用匹配给 b)
        let c = &a;
        assert_eq!(b,c); //Ok

        let mut a = [1,2,3];
        let ref mut b = a; // 等价于 b = &a
        b[0] = 0;
        assert_eq!(a,[0,2,3]); //Ok

        // match 表达式
        fn check_optional(opt: Option<i32>){
            match opt {
                Some(p) =>println!("has value{}",p),
                None => println!("has no value"),
            }
        }
        // fn handle_result(res: i32) -> Result<i32,dyn Error>{
        //     do_something(res)?;
        //     // 问号操作符等价于
        //     match do_something(res){
        //         Ok(o) => Ok(o),
        //         Err(e) => return SomeError(e),
        //     }
        // }

        // 不加语法糖的 match 模式匹配
        fn f01(x: &Option<String>){
            match x {
                &Some(ref s) => {
                    println!("{:?}",s) // 使用 ref 将 s的引用匹配到 函数体中，因此就不必将 s 所有权转移到函数体 {} 中
                },
                &None => {
                    println!("nothing")
                }
            }
        }

        // 加语法糖的 match 模式匹配
        fn f02(x: &Option<String>){
            match x {
                Some(s) => {println!("{:?}",s)},
                None => { print!("is null")}
            }
        }

        // 普通数组
        let arr = [1,2,3];
        match arr {
            [1,_,_] => println!("starts with one"), // 最先匹配到这个,后续的不再匹配了 (输出: starts with one)
            [a,b,c] => println!("starts with something else"), // 注释上段代码，则匹配到这个分支
             _ => println!("nothing") // 注释上段代码，则匹配到这个分支
        };

        // 动态大小数组
        let v = vec![1,2,3];
        match v[..] {
            [a,b] => {/* 不匹配 */}
            [a,b,c] => {println!("{},{},{}",a,b,c)}
            _ => {/* 必须包含这条分支，因为长度是动态的*/}
        };

        let x = &Some(3);
        if let Some(y) = x { // 这里编译器自动将 Some(y) 填充为 &Some(y)
            y; // &i32
        }
    }

    /* 智能指针
    1、值语义(实现Copy trait): 可以存储在栈内存的基本数据类型，在语义层面基本数据类型就是一种值；按值来传递给其他变量、函数，也就是全部数据进行传递。

    2、指针语义(实现Move trait): 在运行时动态增长的类型 (动态数组、动态字符串)；将存储在栈上的指针传递给其他变量、函数，也就是只传递栈上指针。
     */

    #[test]
    fn test_37(){
        let x = Box::new(42); // 这里 x 是一个智能指针
        let y = *x;  // 取 x 这个指针指向的值: 42
        assert_eq!(y,42);//Ok

        // 自动解引用: 点调用操作
        struct User{
            name: &'static str,
        }

        impl User{
            fn name(&self){
                println!("{:?}",self.name)
            }
        }

        let u = User{name: "Alex"};
        let y = Box::new(u);
        y.name();

        // 自动解引用: 函数参数
        fn takes_str(s: &str){
            println!("{:?}",s)
        }

        let s = String::from("hello world");
        takes_str(&s); // 因为 String 类型实现了 Deref trait，所以 String 能自动解引用为 &str 类型
    }

    #[test]
    fn test_38(){
        // let x : &Vec<i32>;
        // {
        //     let y = Vec::new();
        //     x = &y;
        // }
        // println!("x's length is {}",x.len());
    }

    // Move 语义 borrow 的使用
    #[test]
    fn test_39(){
        struct Hello {
            v: i32,
        }
        impl Drop for Hello{
            fn drop(&mut self) {
                println!("drop borrow1")
            }
        }
        fn consume(h: Hello){
            println!("{:?}",h.v)
        }
        fn print_hello(h: &Hello){
            println!("{:?}",h.v);
        }
        // {
        //     let h = Hello{v: 300};
        //     println!("{:?}",h.v);
        //     consume(h); //Ok h的所有权已经被转移到了 consume 内
        //     consume(h) //Error 这里再次使用会报错，因为 h 在上个函数就被消耗掉了
        // }

        {
            let h = Hello{v: 300};
            println!("{:?}",h.v);
            // h 这个主体派生出 N 多个借用(引用),h 被消耗掉，就不能派生借用了
            print_hello(&h);//Ok 这里只是使用了 h 的一个借用, h 所有权并未转移到 print_hello() 内，h 未被消耗
            print_hello(&h);//Ok 这里只是使用了 h 的一个借用, h 所有权并未转移到 print_hello() 内，h 未被消耗
        }

        fn return_str()->String{
            let mut s = "Rust".to_owned();
            for i in 0..3{
                s.push_str("Good");
            }
            s
        }
        let aa = return_str();
        println!("{:?}",aa);

        #[test]
        fn test_40(){
            // let r ;
            // {
            //     let x = 5;
            //     r = &x; // r 引用了 x 指向的值
            // } // 在这里 x 被 drop ,同时 x 指向的值也被清理
            // println!("r: {}",r); // 这里 r 就成了无效引用，指向了一个不存在的内存
        }

        #[test]
        fn test_41() {
            // let r ;
            // {
            //     let x = People{
            //         name: "小明".to_string(),
            //         age: 18,
            //     };
            //     r = &x; // r 引用了 x 指向的值
            // } // 在这里 x 被 drop ,同时 x 指向的值也被清理
            // println!("r = {:?}",r); // 所以这里就发生了悬垂指针

            let r ;
            {
                let x = People{
                    name: "小明".to_string(),
                    age: 18,
                };
                r = x; // x 的所有权转移给了 r
            } // 在这里 x 被 drop 了，但是 r 获得了 x 的所有权，因此 r 又重新指向了 x 指向的那块内存
            println!("r = {:?}",r);
        }

        // 什么是Sized?
        // Sized是Rust再编译阶段检查对象操作的一个基本依据,
        // Rust只允许操作已知大小的对象, 未知大小的对象只能操作它的指针(&).

        fn sized_correct() {
            #[derive(Debug)]
            struct Water<T>(T);            // 等同于 struct Status<T: Sized>(T);

            #[derive(Debug)]
            struct Cup(Water<i32>);

            let water = Water(10);
            let cup = Cup(water);
            println!("{:?}", cup);         // output: Cup(Water(10))
        }


        // 问题代码, 需要注释掉才能运行.
        //fn sized_error() {
        //    #[derive(Debug)]
        //    struct Water<T>(T);            // 等同于 struct Status<T: Sized>(T);
        //
        //    #[derive(Debug)]
        //    struct Cup(Water<[i32]>);     // 由于[i32] 是一个队列, 因此它是未知大小
        //}

        fn use_unsized_to_fix_sized_error() {
            #[derive(Debug)]
            #[allow(dead_code)]
            struct Bar<T: ?Sized>(T);

            #[derive(Debug)]
            #[allow(dead_code)]
            struct BarUse<'a>(Bar<&'a [i32]>);
        }

        fn thinking() {
            #[derive(Debug)]
            #[allow(dead_code)]
            struct Bar<T: ?Sized>(T);

            #[derive(Debug)]
            #[allow(dead_code)]
            struct BarUse<'a>(Bar<&'a [i32]>);
            // 备注: 虽然这里可以定义Bar<[i32]> ,
            //       但是实际上实现起来不能直接写slice, 因为编译器不允许未知大小的东西编译通过,
            //       解决办法是改成&[i32]

            let s = [1,2,3,4];
            let bar = Bar(&s[0..2]);
            let bu = BarUse(bar);
            println!("{:?}", bu);
            // 写成这样能运行的原因是, &s[0..2]是一个引用, 引用就是一个指针, 指针是固定大小的.
            // 如果指针是固定大小的, 那么上面定义的?Sized对于这个例子来说就没有意义了.
            // 这个问题我现在的水平还无法解决, 等以后水平不断深入在来解决把, 先留个TODO
            // TODO: FIXME.
        }


        fn main() {
            sized_correct();
            // sized_error();
            use_unsized_to_fix_sized_error();
            thinking();
        }

    }
}


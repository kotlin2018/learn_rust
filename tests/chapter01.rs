
#[cfg(test)]
pub mod chapter01{
    use std::collections::BTreeMap;
    use std::fmt::Debug;
    use std::io::Read;
    use std::path::Component::Prefix;
    use std::sync::Arc;
    use hmac::digest::KeyInit;
    use hmac::Hmac;
    use jwt::SignWithKey;
    use sha2::Sha256;

    // 函数项类型
    #[test]
    fn test_01(){
        struct A(i32,i32);
        impl A {
            fn sum(a: i32,b: i32) -> i32 {
                a + b
            }

            fn math(&self) -> i32 {
                Self::sum(self.0,self.1)
            }
        }
        let a = A(1,2);
        let add = A::sum; //A::sum 是一个 Fn item (函数项)
        let add_math = A::math; // A::math 是一个 Fn item
        assert_eq!(add(1,2),A::sum(1,2));
        assert_eq!(add_math(&a),a.math());
    }

    // 函数与闭包
    #[test]
    fn test_02(){
        // 函数无法捕获环境变量
        // fn counter(i: i32) -> fn(i32) ->i32{ // fn(i32) ->i32 是一个函数指针，函数可以用一个变量来绑定，因此函数也可以是指针类型
        //     fn inc(n: i32)-> i32 {
        //     n + i // 这里 inc 无法捕获 counter()函数的参数 i
        // }
        //     inc // 返回这个函数指针
        // }

        // 使用闭包捕获环境变量
        fn counter(i: i32) -> impl FnMut(i32)->i32{  // FnMut 是个 trait
            // println!("{}",n);
            println!("{}",i);
            move |n| n+i // 这是一个闭包，它的类型是 FnMut(i32)->i32
        }

        // 先调用函数,该函数返回了一个闭包
        // mut 声明这是一个可变闭包
        let mut closure = counter(1);
        // 再调用闭包
        assert_eq!(3,closure(2));
    }

    // 闭包与函数指针互通
    #[test]
    fn test_03(){
        type RGB = (i16,i16,i16);
        fn color(c: &str) -> RGB {
            (1,1,1)
        }

        // show 的入参是函数指针
        fn show(c: fn(&str) -> RGB){
            println!("{:?}",c("black"))
        }

        let rgb = color;
        show(rgb); // (1,1,1)
        // 定义了实现 `Fn(&str) -> RGB` trait 的闭包类型
        let c = |s: &str|{(1,2,3)};
        show(c);
    }

    // 闭包的实现原理
    #[test]
    fn test_04(){
        // 未捕获环境
        let c1 = ||println!("hello");
        c1();

        // 捕获了环境变量，但是未修改环境变量
        let ans = 42;
        let c3 = ||{
            println!("{}",ans) //闭包的这个函数体直接捕获了外部变量 ans
        };
        c3();

        // 捕获环境变量，并修改环境变量
        let mut arr = [1,2,3];
        let mut c2 = |i|{
            arr[0] = i; // arr[0] 这个变量指向 指向(绑定) i 这个值
            println!("{:?}",arr);
        };
        c2(0);
    }

    // 逃逸闭包: 能被函数返回，不在函数调用过程中被销毁的闭包。
    #[test]
    fn test_05(){

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
        //     // 但是此时 s 的指针又被 move 进了闭包，如果 s 被闭包带出了函数，此时 s就成了悬垂指针。
        //     // 这 move 是可有可无的，s 没有实现 Copy，即使不使用 move，闭包捕获这个 s 也会自动转移 s 的所有权
        //
        //     let mut s = "hello".to_string();
        //     move |i|{s += i;s}
        // }
        // let i = "world";
        // let mut arr_closure = c_mut2(); // Error

        // 即使不使用闭包，也无法返回一个局部变量的引用，主要是为了防止出现悬垂指针。
    }

    // rust编译器 不允许使用被闭包捕获的引用
    // [唯一不可变引用]: 被闭包捕获的引用
    #[test]
    fn test_06(){
        // [唯一不可变引用]: 被闭包捕获的引用
        let mut a = [1,2,3];
        let x = &mut a;

        // 结论: rustc 不允许你去使用被闭包捕获的引用
        // let mut c = ||{(*x)[0] = 0;};
        // let y = &x; //Error
        // c();
        // let z = &x; //OK
    }


    #[test]
    fn test_07(){
        // 闭包实现 Copy/Clone 的两条规则
        // 1、如果环境变量实现了Copy，闭包如果以可变借用方式捕获环境变量，并对其进行修改，则闭包自身不会实现Copy
        // ( 如果闭包对环境变量产生影响，这个闭包自身就不能实现Copy
        // 如果这个闭包能实现Copy，相当于多个闭包来对环境变量进行修改，这违反Rust可变借用的规则

        // 2、如果环境变量自身是 Move 语义，则闭包内捕获环境变量的操作涉及修改环境或者消耗环境变量，则闭包自身不会实现Copy

        // 实现 Sync/Send 的三条简单规则
        // 1、如果所有捕获变量均实现了Sync，则闭包实现Sync
        // 2、如果环境变量都不是 [唯一不可变引用] 方式捕获的，并且都实现了 Sync，则闭包实现Send
        // 3、如果环境变量是以 [唯一不可变引用]、[可变引用]、Copy或Move所有权捕获的，那闭包实现Send
        fn foo<F: Fn() + Copy>(f: F){}

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
    fn test_08(){
        let s = "hello";
        let mut c = ||println!("{:?}",s);
        c();
        c();
    }

    #[test]
    fn test_09(){
        // let a = 1;   // a 是 i32 类型，默认实现了 Copy trait,并且 a 指向的值是存储在栈上，因此 a 绑定的值可以被修改，计算
        // let b = a+1;
        // println!("{}",a); // 1
        // println!("{}",b); // 2

        // let c = Box::from(3); // Box 在堆上开辟了一块内存，并存储了值 3，变量 c 绑定了这块内存，(存储在堆上的类型只实现了 Move trait)
        // let d = &c; // 将 c 的引用传递给 d，c的所有权还是在c自身
        // println!("{}",c); // 3
        // println!("{}",d); // 3

        // 移动所有权
        // let c = Box::from(3); // Box 在堆上开辟了一块内存，并存储了值 3，变量 c 绑定了这块内存，(存储在堆上的类型只实现了 Move trait)
        // let d = c; // c的所有权已经转移到了d，c已经被消耗掉了
        // println!("{}",c); // Error
    }

    // 高阶生命周期 (高阶trait限定)
    // for<> 语法
    #[test]
    fn test_10(){
        trait DoSomeThing<T> {
            fn do_sth(&self,value: T);
        }
        impl<'a,T: Debug> DoSomeThing<T> for &'a usize {
            fn do_sth(&self, value: T) {
                println!("{:?}",value);
            }
        }

        // 错误的写法
        // fn foo<'a>(b: Box<dyn DoSomeThing<&'a usize>>){
        //     let s: usize = 10;
        //     b.do_sth(&s)
        // }

        // late bound 延后限定
        // for<'f> 就是告诉编译器，只有调用 do_sth 时才去判断 &s 的生命周期
        fn bar(b: Box<dyn for<'f> DoSomeThing<&'f usize>>){
            let s: usize = 10;
            b.do_sth(&s);
        }

        let x = Box::new(&2usize);
        //foo(x);
        bar(x);
    }

    // 生命周期参数: 闭包
    #[test]
    fn test_11(){
        // let f = |x: &i32| x; // Error
        // let i = &3;
        // let j = f(i);

        // 修复上述错误
        // late bound
        fn annotate<T,F>(f: F) -> F where for<'a>F: Fn(&'a T) -> &'a T {  // 这个 for<'a> 限定说明: 后面那个函数的 入参生命周期必须大于返回值
            f
        }
        let f = annotate(|x|x);
        let i = &3;
        let j = f(i); // 入参 i 的生命周期 > 返回值 j 的生命周期 (满足 f 这个闭包的生命周期限定)
        assert_eq!(*j,3);

        // early bound
        fn annotate02<'a,T: 'a,F>(f: F)-> F where F: Fn(&'a T) -> &'a T{
            f
        }

        let f01 = annotate02(|x|x);
        let i01 = &3;
        let j01 = f01(i01);
        assert_eq!(*j01,3);
    }

    #[test]
    fn test_12(){
        // trait CheckSum<R: Read>{
        //     fn calc(&mut self,r: R) -> Vec<u8>;
        // }
        //
        // struct Xor;
        // impl<R: Read> CheckSum<R> for Xor {
        //     fn calc(&mut self,mut r: R) -> Vec<u8> {
        //         let mut res: u8 = 0;
        //         let mut buf = [0u8;8];
        //         loop {
        //             let read = r.read(&mut buf).unwrap();
        //             if read == 0 {
        //                 break;
        //             }
        //             for b in &buf[..read]{
        //                 *res = b;
        //             }
        //         }
        //         vec![res]
        //     }
        // }
    }


    // trait bound 的用法
    #[test]
    fn test_13(){
        struct Person;
        trait Behavior{
            fn sleep(&self){ // trait 的默认实现
                println!("sleep!");
            }
        }
        impl Behavior for Person{}

        fn sleep(t: impl Behavior){
            t.sleep();
        }

        fn generic_sleep<T: Behavior>(t: T){
            t.sleep();
        }
        sleep(Person);
        generic_sleep(Person);
    }

    #[test]
    fn test_14(){
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("sub","someone");

        let token_str = claims.sign_with_key(&key).unwrap();
        println!("{:?}",token_str);
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod test{
    use super::*;

    //! 所有权与类型系统，不同的类型拥有不同的行为 trait，例如:Copy(),Move()
    //!
    //! 资源最多只能被使用一次
    //!
    //! 移动语义(Move)  资源 -----消耗------>资源
    //!
    //! 在运行时，动态增长的类型是被分配到堆内存，例如:动态增长的字符串、数组，同时栈上保存了一个指向该堆内存的指针，
    //! 如果该类型实现了 Copy 语义，则栈上会有两个指针来指向这同一块堆内存，相当于有两个指针来控制同一块堆内存，这是不安全的。
    //! 因此 动态增长的类型只能实现 Move 语义。
    //!
    //! 在运行时，
    //!
    //!
    //! 复制语义(Copy)  资源 -----复制------>资源
    //!
    //! 所有权语义下的内存管理方式:
    //!
    //! 1、默认存储数据，是存储在栈上。
    //!
    //! 2、利用栈来自动管理堆内存
    //!
    //! (当函数调用结束的时候，定义在函数内部的本地变量会被清空，放在栈上的，指向堆内存的指针会被清理，同时该指针指向的堆内存也会被清理)
    //!
    //!

    #[test]
    fn test_ownership(){

    }
}

#[cfg(test)]
mod test {

    // 写了下面的注释 + 代码后，在当前 project 终端执行 cargo doc --open (生成文档并打开文档) 或者 (cargo doc: 生成文档)

    /// 生成 cargo doc
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ````
    /// let arg = 5;
    /// let answer = add_one(arg);
    ///
    /// assert_eq!(6,answer);
    /// ````
    pub fn add_one(x: i32) -> i32 {
        x +1
    }
}
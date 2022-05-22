# 1、[安装`rust`](https://www.rust-lang.org/zh-CN/tools/install)
````shell script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

# 2、[cargo-edit 工具安装](https://www.cnblogs.com/gelare/p/12711270.html)
* ````shell script
  cargo install cargo-edit
  ````
  
* 可用子命令:
````shell script
# 添加依赖到当前 package 的 Cargo.toml 文件中

cargo add 
````
````shell script
# 删除 当前 package 的依赖 (即:删除 Cargo.toml 文件中的依赖项)
cargo rm 
````
````shell script
# 升级 当前 package 的依赖 (即:更新 Cargo.toml 文件中的依赖项)
cargo upgrade
````

# 3、[rust 可选插件](https://zhuanlan.zhihu.com/p/89932207)
````shell script
cargo install cargo-make
````
````shell script
cargo intall cargo-cache
````
````shell script
cargo install cargo-audit
````

# 4、[Cargo 命令](https://jishuin.proginn.com/p/763bfbd70e6c)
* ````shell script
  # 构建和安装 Rust 二进制文件 默认从 crates.io 下载，可选安装源 --git, --path, --registry
  
  cargo install 
  ````
  
* ````shell script
  # 删除一个用 cargo install 安装的包
  
  cargo uninstall
  ````
  
* ````shell script
  # 显示当前终端下的依赖关系树
  
  cargo tree
  ````
  
* ````shell script
  # 搜索 crate, 返回 crate 的描述信息
  cargo search
  ````

* ````shell script
  # 输出 应用于当前 crate 的宏扩展结果和 #[derive] 扩展
  
  cargo expand
  `````



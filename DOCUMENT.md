## learn rust

## 简介
#### rust 优势
>* 高性能
>* 安全
>* 多线程

#### rust 擅长应用
>* web service
>* web service
>* 嵌入式
>* 系统工具

#### rust 安装
>* www.rust-lang.org/tools/install 安装 rust
>* rustup update
>* rustc --version
>* rustup doc

#### rust 代码风格
>* 后缀名 .rs 文件
>* 明明规范 hello_world.rs
>* rust 的缩进是 4 个 space，不是 tab

#### rust 编译
>* rustc "源文件路径"， 生成与源文件透明的 .exe 文件
>* .pdb 文件保存调试信息，windows 平台
>* rustc 只适合简单的 rust 项目

#### cargo 管理
>* 构建系统，包管理工具
>* cargo 随着 rust 安装而安装
>* cargo new "项目名称" 创建 rust 项目，例如，cargo new wazzup
>* cargo init 用来初始化一个文件夹，变成 rust 项目
>* src 用来放源代码，main.rs 中包含主函数
>* .toml 是 cargo 的配置信息
>* cargo build，cargo build --release
>* cargo run
>* cargo check

##### .toml 文件
>* package 表示本模块的信息
>* dependency 表示依赖的第三方 rust 库，称作 crate

## 语法
#### 引用外部库
>* use 关键字，例如 use std::io; 表示引用 std 库中的 io 模块

#### 变量
>* let 关键字用于声明变量，例如 let a = 10;
>* 默认情况下，变量是不可变的，即 immutable，如果要让变量初始化后可以再被修改，需要使用 mut 关键字，例如， let mut a = 10;
>* 引用 (地址)，通过 & 符号获得，例如 &a，默认情况下，引用也是不可变的，如果要通过引用修改内存内容，需要在获得引用时，使用 &mut 来指向变量名，例如 &mut a;

#### 关联函数
>* :: 类型域的函数，类似 cpp 中的静态函数，例如 String::new();

## 内部类
#### String
>* utf-8 编码的字符串类型
>* 根据需要自动扩展自身大小


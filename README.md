# rust-calendar
一个使用rust编写的日历，可以打印某个月的日历  
适合初学者学习此项目
```
A calendar command-line tool written in rust
Usage: rust-calendar [ARGUMENTS]
Arguments:
    now             Prints the calendar for the current month
    debug_calendar  Debug prints the calendar for the current month
    help            Show this help
```
## 编译
```shell
cargo build
```
## 运行
```shell
cargo run
```
## 效果
```shell
Sun     Mon     Tue     Wed     Thu     Fri     Sat
1       2       3       4       5       6       7
8       9       10      11      12      13      14
15      16      17      18      19      20      21
22      23      24      25      26      27      28
29      30      31      0       0       0       0
0       0       0       0       0       0       0
```

## 安装rust的相关链接
1. [在 Windows 上通过 Rust 进行开发的概述 | Microsoft Learn](https://learn.microsoft.com/zh-cn/windows/dev-environment/rust/setup)
2. [在 Windows 上针对 Rust 设置开发环境 | Microsoft Learn](https://learn.microsoft.com/zh-cn/windows/dev-environment/rust/setup)
3. [知乎 windows10 安装rust](https://zhuanlan.zhihu.com/p/183941666)
4. [win10安装rust的开发环境完整教程（gcc编译器和vc++编译器）| 博客园](https://www.cnblogs.com/qumogu/p/16412144.html)
5. [菜鸟教程Windows Rust 环境搭建](https://www.runoob.com/rust/rust-setup.html)
6. [知乎Rust的安装（Mac系统）](https://zhuanlan.zhihu.com/p/104634073)
7. [知乎在国内用 mac 安装 Rust](https://zhuanlan.zhihu.com/p/587320478)
8. [Linux上安装rust - 知乎](https://zhuanlan.zhihu.com/p/308452799)
9. [如何在linux上安装rust语言 - 知乎](https://zhuanlan.zhihu.com/p/391842442)
10. [如何在 Ubuntu 和其它的 Linux 发行版安装 Rust 和 Cargo - linux中国](https://linux.cn/article-13938-1.html)
11. [在Termux(Android)上安装Rust | Level Up](http://larrynung.github.io/2019/06/28/Rust-Install-on-Termux/)

注意事项：Rust编译器需要C语言编译器，否则无法正常运行。

如果编译速度很慢，卡在下载的地方了怎么办？  
可以尝试更换cargo源来提升速度  
[更换Cargo源 - w3cschool](https://www.w3cschool.cn/cargo_guide/cargo_guide-uxdg3l62.html)
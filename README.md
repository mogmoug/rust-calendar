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
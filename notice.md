1. 一对大括号声明了代码块(block),块有自己的作用域:
2. 块也是表达式，意味着它的计算结果是一个值:
    let x = { 42 };
3. let x =1;
  let x=2 ;
  是如何隐藏上一个的
4. 函数块的最后省略分号意味着返回这个值，例如下面两个函数功能是一样的:
5. return 4;    4
6. if 条件是表达式
fn fair_dice_roll() -> i32 {
    if feeling_lucky {
        6
    } else {
        4
    }
}
7. 用&str作为参数比&String更加通用
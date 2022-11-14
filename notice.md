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
8. struct省略语法 、更新语法 ..user1
  Tuple stuct 整体有个名，但里面的元素没有名
  struct Color(i32,i32,i32);
  Unit-Like Struct 空的结构体
  struct 的所有权 实例有效字段数据有效
  使用生命周期保证引用是有效的
9. struct 方法
  在struct(enum,traitd对象)的上下文中定义
  方法的第一个参数总是self，表示正在被调用的struct的实例
  不用传参数，有更好的组织性
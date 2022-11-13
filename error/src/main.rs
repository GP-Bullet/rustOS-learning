//panic! 宏执行 打印错误信息 展开(unwind)、清理调用栈(stack) 退出
//中止（abort）



//处理Result的一种方式 : match表达式
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file)=>file,//里面是一个变量
        Err(error)=>{
            panic!("Error opening file {:?}",error)
        }
    };
    
    //相当于上面的缩写，若为ok 返回ok的值，如果Err调用panic！宏
    let f = File::open("hello.txt").unwarp();
    //但是不能指定错误信息，  但是expect可以


}
//闭包（closure） Result<T,E>有很多方法 闭包 省去很多match的嵌套
// unwarp 


//传播错误 将错误返回给调用者
// ?传播错误的快捷方式 把？作用于Ruselt
//如果Rusult是ok：ok的值就是表达式的结果，然后继续执行程序
//如果是Err Err就是整个函数的返回值，就像使用了return 
//针对不同错误原因，返回同一种错误类型
//链式调用

//main 返回一个trait对象 Box<dyn Error  任何可能的错误类型
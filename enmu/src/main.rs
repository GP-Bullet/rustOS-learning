enmu IpAddKind{
   V4(u8,u8,u8,u8),      //两个变体V4和V6
   V6(String),    //不用额外的struct，然后数据类型的指定很自由
}                   //可以是字符串结构体 另一个枚举

//自定义的类型 
fn main(){
    let home = IpAddKind::V4(127.0.0.1);
    let loopcack = IpAddKind::V6(String::from("::1"));

}
//允许数据直接附加到枚举的变体中

//用impl 为枚举定义方法


enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32, i32),
}
impl Message{
    fn call(&self){   }
}
fn main(){
    let q = Message::Quit;
    let m = Message::Move{x:12,y:24};
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0,255,255);

    m.call()

//!Option 枚举 在prelude中 
//!  描述了某个值可能存在（或者是某种类型）或不存在的情况
//! Rust 没有NULL 
//!Option<T>和T是不同的类型，不可以把Option直接当成T
//手动转换

//match 模式可以是字面值、变量名、通配符
match coin{//coin是一个表达式
    Coin::penny=>1,//左边是一个待匹配的模式  右边是对应的代码，简单=>
    Coin::Nickel=>5,//匹配成功的表达式作为match表达式的结果
    Coin::Dime=>{
        println!("Dime");
        1
    } 
    Coin::Quarter=>25.
}

//绑定值的模式  

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        _=>(),//什么也不发生
    }
}
fn main(){
    let c = Coin::Quarter(UsState::Alaska);
    println!("{}",value_in_cents(c));
}

//!匹配Option<T>
//! match必须穷举所有的模式 用_通配符：替代其余没有列出的值


//if let
fn main(){
    let v=Some(0u8);
    match v{
        Some(3)=>println!("three"),
        _=>(println!("other"));
    }
}
lf let Some(3)=v{
    println!("three");
} else{
    println!("others");
}
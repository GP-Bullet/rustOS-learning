#[derive(Debug)]
struct Fang{
    chang:u32,
    kuan:u32,
}
//定义方法需要一个块
impl Fang{
    fn area(&self)->u32{//&mut 获得所有权
        self.chang*self.kuan
    }
    // fn can_hold(&self,other:&Fang)->bool{
    //     self.chang>other.chang && self.kuan>other.kuan 
    // }
    // //关联函数通常适用于构造器，即构造实例  ：：
    // //模块创建的命名空间     
    // fn square(size:u32)-。Fang{
    //     Fang{
    //         chang:size,
    //         kuan:size,
    //     }
    // }
}
impl Fang{
    // fn area(&self)->u32{//&mut 获得所有权
    //     self.chang*self.kuan
    // }
    fn can_hold(&self,other:&Fang)->bool{
        self.chang>other.chang && self.kuan>other.kuan 
    }
    //关联函数通常适用于构造器，即构造实例  ：：
    //模块创建的命名空间     
    fn square(size:u32)->Fang{
        Fang{
            chang:size,
            kuan:size,
        }
    }
}
fn main(){
    let _s=Fang::square(20);

    let test1=Fang{
        chang:20,
        kuan:30,
    };
    let test2=Fang{
        chang:30,
        kuan:40,
    };
    let test3=Fang{
        chang:15,
        kuan:20,
    };
    println!("{}",test1.area());
    println!("{}",test1.can_hold(&test2));
    println!("{}",test1.can_hold(&test3));
    println!("{:#?}",test1);
}

//lf let 简单控制流 处理只关心一种匹配而忽略其他匹配的情况

fn main(){
    let v=Some(0u8);
    match v{
        Some(3)=>println!("three"),
        _=>()
    }
}
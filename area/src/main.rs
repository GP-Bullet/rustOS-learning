#[derive(Debug)]
struct Fang{
    chang:u32,
    kuan:u32,
}
fn main(){
    let test=Fang{
        chang:20,
        kuan:30,
    };
    println!("{}",area(&test));
    println!("{:#?}",test);
}

fn area(mian:&Fang)->u32{
    mian.chang*mian.kuan
}
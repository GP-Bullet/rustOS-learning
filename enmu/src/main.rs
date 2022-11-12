enmu IpAddKind{
    V4(u8,u8,u8,u8),      //两个变体V4和V6
    V6(String),    //不用额外的struct，然后数据类型的指定很自由
}
//自定义的类型 
fn main(){
    let home = IpAddKind::V4(127.0.0.1);
    let loopcack = IpAddKind::V6(String::from("::1"));

}
//允许数据直接附加到枚举的变体中
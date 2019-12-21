
use std::io;
fn main() {

println!("calculator is Start  Do your calculations ..... : ");
let mut s = String::new();
let mut v :Vec <f32>= Vec::new();
io::stdin().read_line(&mut s).expect("Failed to read line");
let mut o=" ";
for i in s.split(' '){
    let int: f32 = match i.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

if i=="+"||i=="-"||i=="*"||i=="/"||i=="^"{
    o=i;
}
else{
v.push(int);
}
}
let a:f32=v[0];
let b:f32=v[1];

let r = match o {
"+"=>a+b,
"-"=> a-b,
"*"=> a*b,
"/"=> a/b,
"^"=>a.powf(b),
_=>0.0,
};

println!("{} {} {} = {}",a,o,b,r);










}

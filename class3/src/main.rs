
// use std::thread;
// use std::time::Duration;

// fn main() {


//     thread::spawn(||
//     {
//         for i in 0..5
//         {
//     println!("first thread");
//     thread::sleep(Duration::from_millis(1));
// }});
// for j in 0..5{
// println!("seconnd thread");
// thread::sleep(Duration::from_millis(1));
// };



// }

// thread: sub division o fprocess
// deadlock ak code doosre code ko access krna chahe or na kr ke ye condition deadlock kehlata he or proces waiting situation pr chla jata he
// main thread jese he complete ho ga sare thread kill ho jaen ge or programme exit hoo jaee ga
// octa core ki app octacor me hi speed se run rke gi


//  without time duration
use std::thread;


fn main(){

let handle =thread::spawn(|| {
    
    for i in 0..30{
        println!("{} in first thread ", i);
    }
});
//handle.join(); //yhan phele ye wala thread chle ga

for j in 0..30{
    println!("{} in second thread ", j);

} 

handle; // agr ap ko code paralllel chlana he to custom thread ko end me call kren ge
}


// time k zrye threads ki parallel  ko control kr skte hen 
// loser bydfault mutable rfrence leta he
// fnMut:  function capturing environment variable closer bhi mut ho ga
// fnonce : variable move ho rha ho or ak baar hi call ho ga
// x.meo() method
// y::meo()
891114
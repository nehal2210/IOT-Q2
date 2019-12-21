// fn main() {
// let numbers=vec![100,20,30,40,59,45,72];
// let mut largest=numbers[0];
// for i in numbers{
//     if i>largest{
//         largest=i;
//     }
// }

// println!("{}", largest);

// }


// largest integer k lioa alg function larest character k lai alg functin largest float k lia alfg function is msle ka hl
// generics type hen generic function sb ko dekh lle ga.
// compile hote wat rust ko pata hona chahiye  k tmamam variables ki data type kia he
// T ak place holder jhan wo data type place kr de ga


// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main(){
    





//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);

// }



// #[derive(Debug)]
// struct Point<T,U>{ //yhan ho skta he x me alg dtype or y me alg dtype ae to is k lia do generics ki zarooraat he.{
//     x: T,
//     y: U,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };

// println!("{:?}",float );


// }



// e.g: code

// #[derive(Debug)]
// struct Patient<T,U>{ //yhan ho skta he x me alg dtype or y me alg dtype ae to is k lia do generics ki zarooraat he.{
//     name:String,
//     age: T,
//     salary: U,
// }

// fn main() {
//     // let integer = Point { x: 5, y: 10 };
//     // let float = Point { x: 1.0, y: 4.0 };

// let p1=Patient{
//     name:String::from("Ahmed"),
//     age:82,
//     salary:String::from("thirty thousands"),

// };

// let p2=Patient{
//     name:String::from("hamad"),
//     age:String::from("forty"),
//     salary:23,
    
// };


// println!("{:?}",p1 );

// println!("{:?}",p2 );

// }

// genric using methods: 

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }



struct Point<T, U> {
    x: T,
    y: U,
}
// p1 ka x or p2 ka y 
// jb instance bnate ho to generic us data type se bouns ho jata he
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
waiting 
fn main() {
let name = String::from("Umair");
let name_length = name.len();
println!("{}",name_length);
let age = [20,22,24,26];
//element 0 1 2 3 4 5 6 7 8 9
let temp = 9;
println!("Age is : {}",age[temp]);
//age.9 //tuple calling
//age[3] //array calling
// age[9]
//println!("Age is : {:#?}",age.get(2));


}

#[derive(Debug)]
enum Direction {
Right,
Left,
Up(String),
Down(u8)
}

impl Direction {
fn impl_ppt(&self) {
// &Direction Readable
// Direction Move
// &mut Direction writeable
println!("We are in impl_ppt Method");
println!("{:?}",self);
}

fn impl_new(faisal:u8)->Direction{
Direction::Down(faisal)
}

}

//fn ppt(age:u32,name:String,salary:u32) {

fn ppt(data:Direction) {
println!("We are in ppt Function");
println!("{:?}",data);
}

fn new1(faisal:u8)->Direction{
Direction::Down(faisal)
}

fn main() {
let age : u8 = 55; //enum
let age_data = [22,24,26,28]; //struct 
let d1 = Direction::Right;
println!("{:?}",d1);
let d2 = Direction::Left;
ppt(d2);
let d3 = Direction::Up(String::from("Drive Slow")); //string Type
d3.impl_ppt();
println!("from main fn : {:?}",d3);
let d4 = new1(40);
d4.impl_ppt();

Direction::impl_new(20).impl_ppt();



fn main() {
let mut age:u16 = 0;
loop {
if age >5 {
break;
}
println!("Loop : Age is : {}",age);
age = age + 1; 

}

let mut score:u16 = 0;

while score <=5 {
println!("While : score is : {} ",score );
score +=1;
}

//for owais in 0..<5 {
for owais in (0..=5).rev() {
println!("for : value is : {}",owais);
}

// println!("Welcome Batch 4");
// println!("Welcome Batch 4");
// println!("Welcome Batch 4");
// println!("Welcome Batch 4");
// println!("Welcome Batch 4");
// println!("Welcome Batch 4");
// println!("Welcome Batch 4");
// println!("Welcome Batch 4");
// println!("Welcome Batch 4");
// println!("Welcome Batch 4");
}

#[derive(Debug)]
struct Food {
name : String,
price : u16,
type1 : String,
}

fn main() {
let mut food1 = ("Haleem".to_string(),100,"Desi".to_string());
//index 0 1 2
println!("food name : {:?}",food1.0);
food1.0 = "Chicken Haleem".to_string();

println!("food name : {:?}",food1.0);
let food2 = Food {
name : "Paya".to_string(),
type1 : "Desi".to_string(),
price : 250,
};

println!("{:?} ",food2.name);
}

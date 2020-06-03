#[derive(Debug)]
struct life {
  width:u32,
  height:u32,
  }
  
fn main (){
  let rect_1=life{width:50,height:50,};
    let rect_2=life{width:20,height:533,};
      println!("Area Rectacgle1 {}",rect_1.area());
        println!("Area Rectacgle2 {}",rect_2.area());

  let plus_1=life{width:100,height:100};
  println!("This is the Plus Value : {}",plus_1.plus());
  }

  impl life{
  fn area(&self)->u32{
    self.width * self.height
 } 
 fn plus(&self)->u32{
   self.width + self.height
 }
}
//thsi is end hare//
//this is start here
#[derive(Debug)]
struct life {
  width:u32,
  height:u32,
  }

impl life{
  fn area(&self, other:&life)->bool{
    self.width > other.width  &&  self.height > other.height
       }
}  
fn main(){
  let ract_1=life{width:100,height:50};
  let ract_2=life{width:70,height:40};
  let ract_3=life{width:50,height:30};

  println!("{}",ract_1.area(&ract_2));
  println!("{}",ract_1.area(&ract_3));
}
//this is the End hare//
// Start here Associated Funcation//
#[derive(Debug)]
struct life {
  width:u32,
  height:u32,
  }

impl life {
  fn square(size:u32)-> life{
    life{
      width:size,
      height:size,
    }
  }
}

fn main(){

let rect_1=life::square(5);
println!("{:#?}",rect_1);

}


// chapter 5 class code Sir Imran Ali Start Methods and Struct
// chapter 5 class code Sir Imran Ali Start Methods and Struct
// chapter 5 class code Sir Imran Ali Start Methods and Struct
// chapter 5 class code Sir Imran Ali Start Methods and Struct


#[derive(Debug)]
struct Food {
   resturant: String,  //field
   food_item:String,
   size:u8,
   price:u16,
   availablilty:bool
} //blue print

fn main() {

   let pizza = Food {
       resturant: String::from("Pizza Hut"),
       food_item:String::from("Chicken Fajita"),
       availablilty:true,
       price:1500,
       size:16,
   };


   let karahi = Food {
       resturant: String::from("BBQ tonight"),
       food_item:String::from("Chicken Ginger"),
       size:1,
       price:1500,
       availablilty:true
   };
  //  println!("Pizza price {}",pizza.price);
  //  println!("Pizza {:#?}",pizza);
   println!("Karahi {:#?}",karahi);
   pizza.billing("Habib Ullah".to_string());

  //  Food::eid_fitr(1_000);
//    printing(pizza);
//    let mytea=pc();
//    println!("My Tea {:#?}",mytea);
//    println!("geting from pc {:#?}",pc());
}

// fn printing(data:Food){
//    println!("We are in printing function");
//    println!("Pizza price {}",data.price);
//    println!("Pizza {:#?}",data);
   
// }

impl Food {
    fn billing(&self,rider:String){
    println!("We are in billing Method rider id {}",rider);
    println!("Food price {}",self.price);
    println!("Food is here husnain {:#?}",self);
    }
    // fn eid_fitr(eidi:u16){
    //     for mybalance in 0..eidi {
    //         println!("My eidi Balance {}",mybalance);
    //     }
    // }
}

// fn pc()->Food {
//     let chai = Food {
//         resturant: String::from("Pathan"),
//         food_item:String::from("Mix Tea"),
//         size:2,
//         price:100,
//         availablilty:true
//     };
//     chai
// }


#[derive(Debug)]

struct hotel{
  rasturan_name:String,
  food_name:String,
  quantity:u8,
  price:u32,
  avaliblity:bool,
}

fn main(){
  let Piayala_hotel = hotel {
    rasturan_name:String::from("Piyala Hotel"),
    food_name:String::from("Muttoon kharai"),
    quantity:2,
    price:2000,
    avaliblity:true,
  };

  
  // Piayala_hotel.new("Husnain Mirza".to_String());
    Piayala_hotel.new("Habib Ullah".to_string());

    let Wadi_e_meharan = hotel {
      rasturan_name:String::from("Wadi e Mehran"),
      food_name:String::from("Dall Roti"),
      quantity:5,
      price:1200,
      avaliblity:true,
    };
  
    Wadi_e_meharan.new_1("Husnai Mirza".to_string());
    
    //this is the Associated Method >????<<<
    //this is the Associated Method >????<<<
    //this is the Associated Method >????<<<
    hotel::rupee("Husnai Mirza".to_string());  //this is the Associated Method >????<<<  
   //this is the Associated Method >????<<<
}

impl hotel{
  fn new(&self,massage:String){
    println!("Welcome to Piyala Hotel, {}",massage);
    println!("Sir your Bill is : {}",self.price);
    println!("{:#?}",self);

     
  }
fn new_1(&self,cool:String){
  println!("Wellcome to Wade e Mehran {}",cool);
  println!("Food Not Avalible {}", self.avaliblity);
  println!("This just for checkeing : {:#?}",self);
}
//this is the Associated Method >????<<<
//this is the Associated Method >????<<<
//this is the Associated Method >????<<<
//this is the Associated Method >????<<<

fn rupee(eidi:String){
     println!("ya to hoga L=hahah : {}",eidi);
  
} 

}


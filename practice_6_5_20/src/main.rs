fn main (){

    choco(String::from("hello choco"));
    choco(String::from("Ullu"));
    choco(String::from("Heppy Ramadan"));
  
    // this is about Total  
    let number_1: u8=5;
    let number_2:f32=5.5;
    let total=number_1+number_2 as u8;
    println!("Number1 {} Number2 {} and total {}",number_1,number_2, total);
  
    let total_2= number_1 as f32 + number_2;
    println!("{} ",total_2);
  // end total here
  
   println!("Recived {}",rupee());//eidi funcation argument
  
  let height=50;
  let width= 100;
  println!("with multiply {}",area(height,width));
  println!("the value without Total {} {}", height,width); 
  
  // the  Reference and pointing 
  let mut  size=String::from("Husnain");
  let a= &mut size;
  a.push_str(" Mirza");
  println!("{}",a);
  
  // this is the controlflow// 
  
  let b=5;
  if b > 4 {
  
    let mut  counter=0;
      loop{
    println!("The value is false {}",b);
         counter=counter+1;
              if counter == 5{
                 break  
    
    }
         
   }}
  else {
    println!("The value is true {}",b)
  }
  
  
  } /// main function is end here
  
  
  // this is funcation testing
  fn choco(x:String)    {// this is the perameter 
    println!("{}",x);
  }
  
  // this is eidi function
  fn rupee()->u16 {
    println!("Sendign Reupee to Husnain ");
    1000
  }
  
  // this is mulltiply the value 
  fn area (x:u16,y:u16)->u16{
  x * y
  
  }
  
use colored::*;

//declare a structure
struct Book {
  name:&'static str,
  id:u32
}
//declare a trait
trait Printable {
  fn print(&self);
}
//implement the trait
impl Printable for Book {
  fn print(&self){
     println!("Printing book with id:{} and name {}",self.id,self.name)
  }
}

pub fn main(){
  println!("\n{}\n", "Rust Traits".bold().italic().blue());

  //create an instance of the structure
  let b1 = Book {
     id:1001,
     name:"Rust in Action"
  };
  b1.print();
}


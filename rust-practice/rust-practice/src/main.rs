// use serde::{Serialize, Deserialize};
// #[derive(Serialize, Deserialize)]
// struct User{
//   username: String,
//   password: String
// }

// fn main(){
//   let s=String::from("{'username':'alice','password':'1212'}");
//   let u =serde_json::from_str(&s);
//   // println!("Username: {}, Password: {}", u.username, u.password);

// }
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}
use std::fmt::Debug;

struct User{
  username: String,
  password: String
}
impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {{ username: {}, password: {} }}", self.username, self.password)
    }
}
use chrono::Utc;
enum Direction{
  North,
  South,
  East,
  West
}
fn main(){
  let u=User{
    username: String::from("harsh"),
    password: String::from("1234")
  };
  print!("{:?}",u);
    let ans: i32=sum(5, 7);
    println!("The sum is: {}", ans);
    let res=even(ans);
    println!("Is the sum even? {}", res);
    let name: String=String::from("harsh");
     print!("{}",name);
    let v: Vec<i32>=vec![1,2,3];
    println!("{:?}",v);
    let name2=name;
    println!("{}",name2);
    let direction=Direction::West;
  steer(direction);

  let utc=Utc::now();
  print!("Current UTC time is: {}", utc);


      let person = Person {
        name: String::from("John Doe"),
        age: 30,
    };

    // Serialize to JSON
    let json_str = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {}", json_str);

    // Deserialize from JSON
    let deserialized_person:Result<Person, serde_json::Error>  = serde_json::from_str(&json_str);
    match deserialized_person{
      Ok(p)=>println!("Deserialized Person: {:?}", p),
      Err(e)=>println!("Error deserializing: {}", e)
    }

    // println!("Deserialized Person: {:?}", deserialized_person);

    //lifetime example
    let s1=String::from("hello");
    let ans;
    {
      let s2=String::from("world");
      ans=longest(&s1,&s2);
    }
    print!("Longest string is: {}",ans);
}
fn longest<'a,'b>(x:&'a str,y:&'b str)->&'a str{
  let z=y.len();
if(z>0){
  return x;
}
  return x;
}

fn steer(dir:Direction){
  match dir{
    Direction::North=>println!("Steering North"),
    Direction::South=>println!("Steering South"),
    Direction::East=>println!("Steering East"),
    _=>println!("Steering West"),
  }


}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
//borrowing rules
//1) you can have multiple immutable references
//2) you can have only one mutable reference
//3) you cannot have a mutable and immutable reference at the same time
//4) 
fn even(a:i32)->bool{
  return a%2==0;
}

//debug trait is already implemented for boolean,integer,string,vectors etc so we can directly print them using {:?} or {}
// you can implement debug trait for your custom structs as well using #[derive(Debug)]
// you can also implement other traits like Clone, Copy, PartialEq, Eq, Hash etc using #[derive(TraitName)]
// annotations are metadata added to the code to provide additional information to the compiler. it runs at runtime
// macros are used to generate code at compile time. it runs at compile time
// macros are defined using macro_rules! or using the macro keyword
// macros can take variable number of arguments
// procedural macros are of 3 types - Function-like macros, Derive macros, Attribute macros
//copy trait is implmented by numbers and booleans but not by strings and vectors
//clone trait is implemented by strings and vectors but not by numbers and booleans
//copy trait creates a bitwise copy of the data whereas clone trait creates a deep copy of the data
//copying is faster than cloning due to the fact that cloning involves heap allocation





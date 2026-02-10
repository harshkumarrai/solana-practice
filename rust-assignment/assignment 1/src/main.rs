use core::num;
use std::fmt::Error;


trait Serialize{
    fn serialize(&self)->Vec<u8>;
}

trait Deserialize{
    fn deserialize(v:Vec<u8>)->Result<Swap,std::fmt::Error>;
}
#[derive(Debug)]
struct Swap{
    qty_1 :u32,
    qty_2:u32
}

impl Serialize for Swap {
    fn serialize(&self)->Vec<u8> {
        let mut bytes=Vec::with_capacity(8);
        bytes.extend_from_slice(&self.qty_1.to_le_bytes());
        bytes.extend_from_slice(&self.qty_2.to_le_bytes());
        return  bytes;
    }
}
impl Deserialize for Swap {
    fn deserialize(v:Vec<u8>)->Result<Swap,std::fmt::Error>{
        if(v.len()<8)  {
           return Err((std::fmt::Error));
        }
       let num : Vec<u32>=v.chunks(4).map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap())).collect();
        let qty_1=num[0];
        let qty_2=num[1];
        // struct Swap{
        //     qty_1:u32,
        //     qty_2:u32
        // }
        return  Ok((Swap { qty_1, qty_2 }));
    }
}
fn main() {
    println!("Hello, world!");
    let s=Swap{
        qty_1:10,
        qty_2:30
    };
    println!("{:?}",s);
    let v=s.serialize();
     println!("{:?}",v);
    let p=  Swap::deserialize(v);
     println!("{:?}",p);

}

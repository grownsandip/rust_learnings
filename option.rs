fn test_option_type()->Option<u8>{
 let mut op1:Option<u8>=None;
 op1=Some(8);// we cannot directly set option type to integer type but use some(integer type)
 return op1;
}
fn test_option_string()->Option<String>{
    let mut str1:Option<String>=None;
    str1=Some("Sandip".to_string());
    return str1;
}
fn test_option_charType()->Option<CharacterType>{
 let mut chartype:Option<CharacterType>=None;
 chartype=Some(CharacterType::Maze);
 return chartype;
}
pub enum CharacterType{ //defining a enum trait 
    Archer,
    Warrior,
    Maze
  }
  impl ToString for CharacterType{
      fn to_string(&self)->String{
          match self{
              CharacterType::Archer=>"Archer",
              CharacterType::Warrior=>"Wrrior",
              CharacterType::Maze=>"Maze"
          }.to_string()
      }
  }
fn main(){
let res=test_option_type();
println!("{0}",res.unwrap());
let res1=test_option_string();
println!("{0}",res1.unwrap());
let res2=test_option_charType();
println!("{0}",res2.unwrap().to_string());
}
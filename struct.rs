//custom datatypes that we can use to define any real world object
use std::cell::Cell; //this allows us to declare a particular field mutable
pub struct Person<'p>{ //definition converting to generic lifetime
    pub first_name: Cell<&'p str>, //wrpping first_name field to Cell to make it mutable
    pub last_name: String,
    pub birth_year: u16,
    pub birth_month: u8,
}
#[derive(Debug)]
enum VehicleColor{
    Silver,
    Golden,
    Red,
    Blue,
    White,
}
#[derive(Debug)] //using this attribute we can print complex objects without manually specifying them inside println
pub struct Vehicle{
  manufacturer:String,
  model:String,
  year:u16,
  color:VehicleColor
}
#[derive(Debug)]
struct VehicleTuple(String,String,u16);//struct as tuple format data manufacturer,model,year
pub fn new_person()->Person<'static>{
    let p1=Person{first_name:Cell::from("Sandip"),last_name:"Roy".to_string(),birth_month:11,birth_year:1999};//instantiating mut makes all fields mutable which might depend on cases
    p1.first_name.set("Mallik");//set in defined in Cell 
    //p1.last_name="gafur";
    return p1;
}
pub fn new_vehicle()->Vehicle{
    let v1=Vehicle{manufacturer:"Sandip".to_string(),model:"Porsche".to_string(),year:1986,color:VehicleColor::Red};
    return v1;
}
fn new_vehicle2()->VehicleTuple{
    return VehicleTuple("Hunday".to_string(),"Elantra".to_string(),2015);
}
pub fn create_vehicle(){
   println!("{:?}",new_vehicle2());
}
fn main(){
  let myperson=new_person();
  println!("first_name:{0},last_name:{1},birth_month:{2},birth_year:{3}",myperson.first_name.get(),myperson.last_name,myperson.birth_month,myperson.birth_year);
  let mycar=new_vehicle();
  println!("{:?}",mycar);
  create_vehicle();
}
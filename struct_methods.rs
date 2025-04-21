//this file tell about the implementation of methods in the struct
#[derive(Debug)]
enum VehicleColor{
    Silver,
    Golden,
    Red,
    Blue,
    White,
}
#[derive(Debug)]
struct Vehicle{
    manufacturer:String,
    model:String,
    year:u16,
    color:VehicleColor
}
impl Vehicle{ //inside this block we define methods
  fn paint(&mut self,new_color:VehicleColor){ //instance method self to currently referencing object
    //set the color of vehicle instance to something different
    self.color=new_color;//we need to make it mutable &mut is used
  }
  //static methods
  fn create_vehicle()->Vehicle{
    //independent of objects and creates a vehicle whenever called
    let new_vehicle=Vehicle{manufacturer:"default".to_string(),model:"default".to_string(),year:1999,color:VehicleColor::Blue};
    return new_vehicle;
  }
}
fn new_vehicle()->Vehicle{
    let mut myvehicle=Vehicle{manufacturer:"Porsche".to_string(),model:"911".to_string(),year:1999,color:VehicleColor::White};
    myvehicle.paint(VehicleColor::Red);
    return myvehicle;
}
pub fn create_vehicle1(){
    let mut myvehicle=Vehicle::create_vehicle();
    myvehicle.paint(VehicleColor::White);
    println!("{:?}",myvehicle);
}
fn main(){
    create_vehicle1();
}
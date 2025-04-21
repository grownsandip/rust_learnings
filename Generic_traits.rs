//traits defining shared behaviour (like interfaces object) certain methods and function that should be associated
#[derive(Debug)]
struct Person<PetType:Animal+NotDangerous>{ //combining two seperate traits boundaries
 first_name:String,
 pet:PetType,
}
trait Animal{
    //method signature that will be common accross all genrics that inherit this trait
    fn make_sound(&self)->();
} //defining trait any function sintures
trait NotDangerous{}
#[derive(Debug)]
struct Dog{}
impl NotDangerous for Dog { }
impl Animal for Dog{
 fn make_sound(&self)->(){
    println!("Dog barked");
 }
 } //implement function signatures of trait animal with Dog
#[derive(Debug)]
struct Cat{}
impl NotDangerous for Cat { }
impl Animal for Cat{ 
    fn make_sound(&self)->(){
        println!("Cat meows");
     }
} 
#[derive(Debug)]
struct Bear{}
impl Animal for Bear{ //seperate defintion for make_sound signature
    fn make_sound(&self)->(){
        println!("bear roars");
     }
} 
#[derive(Debug)]
struct Tiger {}
impl Animal for Tiger{ 
    fn make_sound(&self)->(){
        println!("tiger roars");
     }
} 
pub fn create_person(){
  let pet1=Dog{};
  let pet2=Cat{};
  let pet3=Bear{};
  let pet4=Tiger{};
  let p1=Person{first_name:"Sandip".to_string(),pet:pet4};
  println!("{:?}",p1);
  p1.pet.make_sound();
}
fn main(){
  create_person();
}
struct Person{
    first_name:String,
    last_name:String
}
pub fn test_closure(){
    let add =|x:i8,y:i8| x+y;//the double pipe character is actually the () in function
    //passing variables in closure donot require data type declaration as compiler autodetects 
    let result=add(23,45);//calling the closure
    let print_res=|| println!("The result is :{}",result);//variables inside the parent block of cloure gets inherited
    print_res();
    let mut p1=Person{first_name:"Sandip".to_string(),last_name:"Roy".to_string()};//converting from static string
    let mut change_name=|new_last_name:&str|p1.last_name=new_last_name.to_string();//the closure also needs to be mutable inorder to change variables
    change_name("Majumdar");
    println!("{}",p1.last_name);
}
fn main(){
    test_closure()
}
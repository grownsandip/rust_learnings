// fn test_if(){
//     let req_age:u8=16u8;
//     println!("Enter the person's age:");
//     let myinput=&mut String::from("");
//     std::io::stdin().read_line(myinput).unwrap();
//     let age=myinput.replace("\n","").parse::<u8>().unwrap();//parsing the taken string into u8 an removing extra newline character from the string input
//     if age>=req_age{
//         println!("Issuing license");
//     }else{
//         println!("No license");
//     }
// }
// fn test_loop(){
//     let mut x:i32=1;
//     loop{
//         if x>=10 {
//             break;
//         }
//         println!("hello from loop");
//         x+=1;
//     }
// }
fn test_for(){
    let arr:[i32;5]=[12,32,16,13,45];
    let age:i32=16i32;
    for value in arr{
       if value>=age{
        println!("You are of age to drive")
       }else{
        println!("You are not of age to drive")
       }
    }
}
fn main(){
    //test_if();
    //test_loop();
    test_for();
}
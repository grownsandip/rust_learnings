pub fn match_test(){
    let age:u16=156;
    match age{
        1..=35=>{
            println!("your age is upto 35")
        }
        150.. =>{
         println!("your age is above 150")
        }
        _=>{
        //default case
        println!("This is the default case")
        }

    }
}
pub fn test_match_array(){
    let prices:[u32;4]=[30000,50000,90000,120000];
    match prices[0..=1]{
        [30000,50000]=>println!("You have some reasonbly priced cars"),
        [30000,50000,..]=>println!("You have all the priced cars"),
        _ =>println!("You donot have any reasonable cars")
    }
}

fn main(){
    match_test();
    test_match_array();
}
pub mod helpers; //publicly including the helper module in main
fn main(){
    let res=helpers::name_helpers::get_full_name("Sandip","Roy");
    println!("Hello i am {0}",res);
}

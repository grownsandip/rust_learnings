pub mod name_helpers{ //neste helper module
    pub fn get_full_name(first:&str,last:&str)->String{
        let f_name:String =format!("{0} {1}",first,last);
         return f_name;
       }
       //by default modules in rust are private to include them in mainrs file we need to declare them as public
}

// pub mod privateModule{
//     //modules under this block are not publicaly visible
// }
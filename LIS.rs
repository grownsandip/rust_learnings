use std::cmp::max;
pub fn lis(str1:&str,str2:&str,idx1:usize,idx2:usize)->usize{
    if idx1==str1.len()|| idx2==str2.len(){
        return 0;
    }
    if str1.chars().nth(idx1).unwrap()==str2.chars().nth(idx2).unwrap(){
        return 1+lis(str1,str2,idx1+1,idx2+1);
    }
    return max(lis(str1,str2,idx1+1,idx2),lis(str1,str2,idx1,idx2+1));
}
pub fn main(){
    let str1:&str="Sandip";
    let str2:&str="Sudip";
    // println!("{:?}",str1.chars().nth(3).unwrap());
    println!("The longest common subsequence is:{0}",lis(str1,str2,0,0));
}
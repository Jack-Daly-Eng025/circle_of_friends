pub struct Solution;

impl Solution {
    pub fn circle_of_friends(n: i32, k: i32) -> i32 {
        
        (2..=n).fold(0, |s, n| (s+k) % n) + 1

    }
}


fn main(){
    let circle_of_friends = Solution::circle_of_friends(5, 6);

    println!("{}", circle_of_friends);
    

}
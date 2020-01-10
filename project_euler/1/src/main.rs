/*Euler 1:
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. 
The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.
WORKS SUCCESFULLY.
*/

fn main() {
    let mut _i: u32 = 0;
    let mut sum: u32 = 0;
    
    for _i in 1..1000 {
        if _i % 3 == 0 {
            sum += _i;
        }else if _i % 5 == 0 {
            sum += _i;
        }
    }
    println!("The sum of all integers multiples of 3 and 5 below 1000 is {}", sum);
}


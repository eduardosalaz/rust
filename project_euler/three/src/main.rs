/*Euler 3:
The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143 ?
WORKS SUCCESFULLY. 
*/

fn main() {
    let mut n:u64 =600851475143;
    let mut i:u64 = 2;
    while i * i < n{
        while n % i == 0{
            n = n / i;
        }
        i += 1
    }
    println!("The largest prime factor of 600851475143 is {}", n);
}

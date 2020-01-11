/*Euler 9:
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a^2 + b^2 = c^2.
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
WORKS SUCCESFULLY but for some reason C is calculated as 425 where it should be 412~. 
*/
use std::f64;
fn main() {
    let mut a:f64 = 3.0;
    let mut b:f64;
    let mut c:f64;
    let limit:f64 = 1000.0;
    let mut product:f64 = 0.0;
    while a < limit {
        b = a + 1.0;
        while b < limit{
            c = (a.powf(2.0)) + (b.powf(2.0));
            c = c.sqrt();
            if (a + b + c) == limit{
                product = a * b * c;
            }
            b += 1.0;
        }
    a += 1.0;  
    }
    println!("The product is {}", product);
}
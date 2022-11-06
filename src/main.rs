use std:: env ;

//the code below finds the modular multiplicative inverse x of of an  


// I explored the rust training here to implement this concepts https://doc.rust-lang.org/book/

//this function finds the greatest common divisor using the eculidean algorithm
//You pass a and b to the function which represents the two numbers whose functions you need to find and it then returns the three values g, x, y
//you can find more information about the euclidean geometry here https://www.math.utah.edu/~fguevara/ACCESS2013/Euclid.pdf
//Also this https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
fn egcd(a:i64,b:i64) ->(i64,i64,i64) 
{
    if a==0
    {
        (b,0,1)
    }
    else
    {
        let(g,x,y)= egcd(b%a,a);
        let d = y - (b / a) * x;
        (g,d,x)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("not enough arguments");
    }

    let (a,b,_c)= egcd(42823,6409);
    println!("Hello, world!={}",a);
}

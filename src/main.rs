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
        (g,d,x) //the returns the response for the egcd
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); //getting the arguments required

    if args.len() < 2 {//error handling to be sure correct number of arguments is passed
        panic!("Not enough arguments");//Panics are explained in detail here: https://doc.rust-lang.org/std/macro.panic.html
    }
    let num1 =args[1].parse::<i64>().unwrap();
    let num2 = args[2].parse::<i64>().unwrap();

    let (a,b,_c)= egcd(num1,num2);
   // println!("Hello, world!={}",a);

    if a==1{
        let modinverse_result:i64 = (b % num2 + num2) % num2;
        println!("The inverse is {}",modinverse_result);
    }
    else{
        println!("The inverse does not exist");
    }
}

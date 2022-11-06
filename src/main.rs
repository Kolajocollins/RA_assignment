//the code below finds the modular multiplicative inverse x of of an  



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

    let (a,b,c)= egcd(42823,6409);
    println!("Hello, world!={}",a);
}

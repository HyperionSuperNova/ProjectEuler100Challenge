fn fibo() -> i128{
    let mut a:i128 = 1;
    let mut b:i128 = 2;
    let mut c:i128;
    let mut sum:i128 = 0;
    while a < 4000000{
        if a % 2 == 0{
            sum+= a;
        }
        c = a + b;
        a = b;
        b = c;
    }
    return sum;
}

fn main(){
    println!("The solution is {}",fibo());
}
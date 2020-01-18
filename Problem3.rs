fn is_prime(n:i128) -> bool{
    if n <= 1{return false;}
    if n <= 3{return true;}
    if n % 2 == 0 || n % 3 == 0{return false;}
    let mut i:i128 = 5;
    while i * i <= n{
        if n%i == 0 || n % (i+2) == 0 {
            return false;
        }
        i=i+6
    }
    return true;
}

fn solution(n:i128) -> i128{
    for _x in (2..((n as f64).sqrt() as i128)).rev(){
        if n % _x == 0{
            if is_prime(_x){
                return _x;
            }
        }
        println!("value of x : {}",_x);
    }
    return -1;
}


fn main(){
    println!("Solution is :{}",solution(600851475143));
}

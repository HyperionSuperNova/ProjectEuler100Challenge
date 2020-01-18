fn solution() -> i64{
    let mut suma:i64 = 0;
    let k3:i64 = ((1000.0/3.0) as f64).ceil() as i64;
    let k5:i64 = ((1000.0/5.0) as f64).ceil() as i64;
    let k15:i64 = ((1000.0/15.0) as f64).ceil() as i64;
    for i in 1..k3{
        suma += 3*i;
    }

    for i in 1..k5{
        suma += 5*i;
    }

    for i in 1..k15{
        suma -= 15*i;
    }
    return suma;
}



fn main(){
    println!("The solution is:{}",solution());
}
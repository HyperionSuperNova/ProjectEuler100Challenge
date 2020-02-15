fn evenly_divisible(gamma:i128) -> bool{
    for i in 1..21{
        if gamma % i != 0{
            return false;
        }
    }
    return true;
}

fn compute_solution() -> i128{
    let mut i:i128 = 2520;
    while !evenly_divisible(i){
        i+=1;
    }
    return i;
}

fn main(){
    println!("The solution is : {}",compute_solution());
}
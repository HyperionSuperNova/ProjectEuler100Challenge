fn is_palindrome(n:i128) -> bool{
    let s: String = n.to_string();
    if s.len() == 0 {return true;}
    let s:Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s[i] != s[j]{ return false;}
        i+=1;
        j-=1;
    }
    return true;
}

fn compute_solution() -> i128{
    let mut max:i128 = 0;
    for i in (100..999 as i128).rev(){
        for j in (100..(i+1) as i128).rev(){
            if is_palindrome(i * j) && (i*j) > max{
                max = i * j;
            }
        }
    }
    return max;
}

fn main(){
    println!("{}",compute_solution());
}
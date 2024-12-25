//write a function is_even that takes a number as input and returns true if its true.


fn main() {
    println!("{}", is_even);
}

fn is_even(num: i32) -> bool {
    if num %2 ==0{
        return true;
    }
    return false;
}

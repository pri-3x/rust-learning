//write a function fib that find the fibonacci of a number it takes as input
//0, 1, 1, 2, 3, 5, 8 


fn main(){
    println!("{}", is_fibonacci(5))
}

fn is_fibonacci(num: i32) -> i32{

  if num<2 {
    return num
    }

    return is_fibonacci(num-1) + is_fibonacci(num-2)
}

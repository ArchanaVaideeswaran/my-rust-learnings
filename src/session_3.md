# Rust Assesmment

## Q-1

Vector is similar to dynamic array. The vector size may grow at run time. 

Answer the following questions for the given code:

1.	Where the vector will be stored: on heap or stack ? `heap`
2.	Does the following code work ? Give reason. `No, Vector v is moved to sum() and droped`
3.	Is it copy or move ? `move`

```code
fn sum(vector: Vec<i32>) -> i32 {
    let mut sum = 0;

    for item in vector {
        sum = sum + item
    }

    sum
}

fn main() {
    let v = vec![1,2,3];
    let s = sum(v);

    println!("sum of {:?}: {}", v, s); 
}
```

## Q-2  

Answer the following questions for the given code:

1.	Where the a and b will be stored: on heap or stack ? `stack`
2.	Does the following code work ? Give reason. `Yes, primitive types are copied hence no error`
3.	Is it copy or move ? `copy`

```code
fn sum(left: i32, right: i32) -> i32 {
    left + right
}

fn main() {
    let a = 42;
    let b = 1;
    let s = sum(a, b);

    println!("this sum of {} and {} is {}", a, b, s); // no error!
}
```
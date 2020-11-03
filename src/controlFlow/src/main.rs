fn main() {
    println!("Hello, world!");
    let mut nums = vec![1, 2, 3, 4];
    // 注意：这里 nums 在第一次函数调用后，被move了

/*
error[E0382]: use of moved value: `nums`
 --> src/main.rs:5:15
 |
3 |     let nums = vec![1, 2, 3, 4];
 |         ---- move occurs because `nums` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
4 |     print_all(nums);
 |               ---- value moved here
5 |     print_ref(nums);
 |               ^^^^ value used here after move
 */

    //print_all(nums);
    //print_ref(nums);
    //print_all_iter(nums);
    //print_all_idx_len(nums);
    double_all(&mut nums); //cannot borrow as mutable, 类型声明的时候必须是mut,才可以转 &mut
    print_all_iter_enumerate(nums);
}

fn print_all(all: Vec<i32>) {
    for a in all {
        println!("{}", a);
    }
}

fn print_ref(all: Vec<i32>) {
    for a in &all {
        println!("{}", a);
    }
}

fn print_all_iter(all: Vec<i32>) {
    for a in all.iter() {
        println!("{}", a);
    }
}

fn print_all_idx_len(all: Vec<i32>) {
    for i in 0..all.len() {
        println!("{}", all[i]);
    }
}

fn print_all_iter_enumerate( all: Vec<i32> ) {
    for (i, a) in all.iter().enumerate() {
        println!("{}:{}", i, a);
    }
}

fn double_all(all: &mut Vec<i32>) {
    for a in all.iter_mut() {
        *a += *a;
    }
}

fn print_some(x: i32) {
    match x {
        0 => println!("x is zero"),
        1 => println!("x is one"),
        10 => println!("x is ten"),
        y => println!("x is something else {}", y),
    }
}
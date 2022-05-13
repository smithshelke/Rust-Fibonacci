use std::time::Instant;
fn main() {
    println!("Enter a number : ");
    println!("Note : Enter a number less than 92 to avoid overflow");
    let mut inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Error while reading input");

    let inp = inp.trim().parse().expect("Error parsing input");
    let mut memoization_store: [u128; 1000] = [0; 1000];
    let mut memoization_store_usize: [usize; 1000] = [0; 1000];

    let now = Instant::now();
    let res = gen_fib(inp);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Nth fibonacci number is {}", res);
    println!("=========");

    let now = Instant::now();
    let res = gen_fib_v2(inp, &mut memoization_store);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Nth fibonacci number using memoization_store USING TYPECAST is {}", res);
    println!("=========");

    let now = Instant::now();
    let inp_as_usize = inp as usize;
    let res = gen_fib_v3(inp_as_usize, &mut memoization_store_usize);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Nth fibonacci number using memoization_store is {}", res);
}

// fib sequence
// 1 1 2 3 5
fn gen_fib(x: u128) -> u128 {
    if x == 1 {
        return 1;
    }
    if x == 2 {
        return 1;
    }
    let mut prev = 1;
    let mut second_prev = 1;
    let mut res = 0;
    for _ in 3..x + 1 {
        res = prev + second_prev;
        second_prev = prev;
        prev = res;
    }
    return res;
}

fn gen_fib_v2(x: u128, memoization_store: &mut [u128; 1000]) -> u128 {
    if x <= 1 {
        return x;
    }
    let idx = x as usize;
    if memoization_store[idx] != 0 {
        return memoization_store[idx];
    }
    let res = gen_fib_v2(x - 1, memoization_store) + gen_fib_v2(x - 2, memoization_store);
    memoization_store[idx] = res;
    return res;
}


fn gen_fib_v3(x: usize, memoization_store: &mut [usize; 1000]) -> usize{
    if x <= 1 {
        return x;
    }
    if memoization_store[x] != 0 {
        return memoization_store[x];
    }
    let res = gen_fib_v3(x - 1, memoization_store) + gen_fib_v3(x - 2, memoization_store);
    memoization_store[x] = res;
    return res;
}

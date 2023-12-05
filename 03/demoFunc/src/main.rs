
fn main() {
    // 1.函数
    // println!("Hello, world!");
    // let x = plus_one(5);
    // println!("x {}", x);
    // if x > 5{
    //     println!("x > 5")
    // }

    // 2.循环
    // loop {
    //     println!("请输入一个数字:");
    // }
    
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("result {}", result);

    // 3.while
//     let mut number = 3;
//     while number != 0 {
//         println!("{}!", number);
//         number -= 1;
//     }

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5{
    //     println!("a[{}] = {}", index, a[index]);
    //     index += 1;
    // }

    // for item in a.iter() {
    //     println!("item {}", item);
    // }
    let a = fib(5);
    println!("a {}", a);
}

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

fn fib(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    // return fib(n - 1) + fib(n - 2);

    // O(n)
    // 空间复杂度 O(n)
    // let mut dp = vec![0; n as usize + 1];
    // dp[1] = 1;
    // dp[2] = 1;
    // for i in 3..=n as usize {
    //     dp[i] = dp[i-1] + dp[i-2]
    // }
    // println!("dp {:?}", dp);

    // v3
    let mut pre = 1;
    let mut pre_pre = 1;
    let mut ret = 0;
    for _i in 3..=n {
        ret = pre + pre_pre;
        println!("ret {} pre {} pre_pre {}", ret, pre, pre_pre);
        pre_pre = pre;
        pre = ret;
    }
    return ret
}

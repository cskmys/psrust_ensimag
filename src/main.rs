const M:usize = 4;
const N:usize = 4;

const MIN:i32 = -100;
const MAX:i32 = 100;

fn print2d(a: [[i32;N]; M]){
    print!("[");
    for i in 0..M{
        print!("{:?}", a[i]);
        if i != M-1{
            println!(",");
        }
    }
    println!("]");

}

use rand::Rng;

fn gen_2d_arr_rand(mut a: [[i32;N]; M]) -> [[i32;N]; M] {
    let mut temp: [i32; M] = [0; M];
    for i in 0..M{
        temp[i] = rand::thread_rng().gen_range(MIN..MAX);
    }
    temp.sort();
    temp.reverse();
    for i in 0..M{
        a[i][0] = temp[i];
    }
    for j in 1..N{
        if a[0][j-1] == MIN{
            a[0][j] = MIN;
        }  else {
            a[0][j] = rand::thread_rng().gen_range(MIN..a[0][j-1]);
        }
    }
    for i in 1..M{
        for j in 1..N{
            let less;
            if a[i][j-1] <= a[i-1][j]{
                less = a[i][j-1];
            } else {
                less = a[i-1][j]
            }
            if less == MIN{
                a[i][j] = less;
            } else {
                a[i][j] = rand::thread_rng().gen_range(MIN..less);
            }
        }
    }
    return a;
}


fn main() {
    let mut a: [[i32;N]; M] = [[0; N]; M];
    a = gen_2d_arr_rand(a);
    print2d(a);
    // println!("0th row: {:?}", a[0]);
    // println!("0th ele: {:?}", a[0][0]);
    // println!("M: {}", a.len());
    // println!("N: {}", a[0].len());
    // println!("0th ele: {}", a[0].first().unwrap());
    // println!("last ele: {}", a[M-1].last().unwrap());
}

const M:usize = 4;
const N:usize = 4;

const MIN:i32 = -100;
const MAX:i32 = 100;

mod lib;

use rand::Rng;

fn print2d(v: &Vec<Vec<i32>>){
    print!("[");
    for i in 0..M{
        print!("{:?}", v[i]);
        if i != M-1{
            println!(",");
        }
    }
    println!("]");
}

fn gen_2d_arr_rand(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let mut temp: [i32; M] = [0; M];
    for i in 0..M{
        temp[i] = rand::thread_rng().gen_range(MIN..MAX);
    }
    temp.sort();
    temp.reverse();
    for i in 0..M{
        v[i][0] = temp[i];
    }
    for j in 1..N{
        if v[0][j-1] == MIN{
            v[0][j] = MIN;
        }  else {
            v[0][j] = rand::thread_rng().gen_range(MIN..v[0][j-1]);
        }
    }
    for i in 1..M{
        for j in 1..N{
            let less;
            if v[i][j-1] <= v[i-1][j]{
                less = v[i][j-1];
            } else {
                less = v[i-1][j]
            }
            if less == MIN{
                v[i][j] = less;
            } else {
                v[i][j] = rand::thread_rng().gen_range(MIN..less);
            }
        }
    }
    return v;
}

fn main(){
    let mut v = vec![vec![0;N];M];
    v = gen_2d_arr_rand(v);
    print2d(&v);
    let mut c = lib::naive_version_2d(&v);
    println!("c {}", c);
    c = lib::recursive_version_2d(&v);
    println!("c {}", c);
    c = lib::recursive_version_2d_opti(&v);
    println!("c {}", c);
}

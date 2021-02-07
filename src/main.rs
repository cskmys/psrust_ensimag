const M:usize = 4;
const N:usize = 4;

const MIN:i32 = -100;
const MAX:i32 = 100;

mod lib;

fn main(){
    let v = lib::gen_2d_arr_rand(M, N, MIN, MAX);
    lib::print2d(&v);
    let mut c = lib::naive_version_2d(&v);
    println!("c {}", c);
    c = lib::recursive_version_2d(&v);
    println!("c {}", c);
    c = lib::recursive_version_2d_opti(&v);
    println!("c {}", c);
}

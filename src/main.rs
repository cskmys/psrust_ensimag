const PATH:&str = "/home/csk/etudes/ps/td/td1/target/fast_tracer/";
const ERR:&str = "failed saving svg file";

mod lib;

fn print2d(v: &Vec<Vec<i32>>){
    let nb_rows = v.len();
    print!("[");
    for i in 0..nb_rows {
        print!("{:?}", v[i]);
        if i != nb_rows - 1{
            println!(",");
        }
    }
    println!("]");
}

fn get_fil_nam(fil_nam: String) -> String{
    let mut path = PATH.to_owned();
    path.push_str(&*fil_nam);
    path.push_str(".svg");
    return path;
}

fn run_func(v: &[Vec<i32>], fil_nam: String, f: &dyn Fn(&[Vec<i32>]) -> i32, tracer: bool) -> i32{
    let mut c = 0;
    if tracer == false {
        c = f(v);
    } else {
        let path = get_fil_nam(fil_nam);
        fast_tracer::svg(path, || {
            c = f(v);
        })
            .expect(ERR);
    }
    return c;
}


fn run_func_lev(v: &[Vec<i32>], lev:i32, fil_nam: String, f: &dyn Fn(&[Vec<i32>], i32) -> i32, tracer: bool) -> i32{
    let mut c = 0;
    if tracer == false {
        c = f(v, lev);
    } else {
        let path = get_fil_nam(fil_nam);
        fast_tracer::svg(path, || {
            c = f(v, lev);
        })
            .expect(ERR);
    }
    return c;
}

fn main(){
    let tracer = false;
    let v:Vec<Vec<i32>>;
    let lev;

    if tracer == true {
        lev = 3;
        v = lib::gen_2d_arr_rand(64, 64, -1000, 1000);
        print2d(&v);
    } else {
        lev = 8;
        v = lib::gen_2d_arr_rand(1024, 1024, -1000, 1000);
    }

    let mut res = vec![0];
    res.pop();


    res.push(run_func(&v,"seq_iter_2d".parse().unwrap(), &lib::seq_iter_2d, tracer));

    res.push(run_func(&v,"seq_iter_2d_opti".parse().unwrap(), &lib::seq_iter_2d_opti, tracer));

    res.push(run_func(&v,"seq_recur_2d".parse().unwrap(), &lib::seq_recur_2d, tracer));

    res.push(run_func(&v,"seq_recur_2d_opti".parse().unwrap(), &lib::seq_recur_2d_opti, tracer));

    res.push(run_func_lev(&v, lev,"seq_recur_lev_2d".parse().unwrap(), &lib::seq_recur_lev_2d, tracer));

    res.push(run_func_lev(&v, lev,"seq_recur_lev_2d_opti".parse().unwrap(), &lib::seq_recur_lev_2d_opti, tracer));

    res.push(run_func(&v,"par_iter_2d".parse().unwrap(), &lib::par_iter_2d, tracer));

    res.push(run_func(&v,"par_iter_2d_opti".parse().unwrap(), &lib::par_iter_2d_opti, tracer));

    res.push(run_func(&v,"par_recur_2d".parse().unwrap(), &lib::par_recur_2d, tracer));

    res.push(run_func(&v,"par_recur_2d_opti".parse().unwrap(), &lib::par_recur_2d_opti, tracer));

    res.push(run_func_lev(&v, lev,"par_recur_lev_2d".parse().unwrap(), &lib::par_recur_lev_2d, tracer));

    res.push(run_func_lev(&v, lev,"par_recur_lev_2d_opti".parse().unwrap(), &lib::par_recur_lev_2d_opti, tracer));

    res.push(run_func(&v,"hyb_iter_2d".parse().unwrap(), &lib::hyb_iter_2d, tracer));

    res.push(run_func(&v,"hyb_iter_2d_opti".parse().unwrap(), &lib::hyb_iter_2d_opti, tracer));

    res.push(run_func(&v,"hyb_recur_2d".parse().unwrap(), &lib::hyb_recur_2d, tracer));

    res.push(run_func(&v,"hyb_recur_2d_opti".parse().unwrap(), &lib::hyb_recur_2d_opti, tracer));

    res.push(run_func_lev(&v, lev,"hyb_recur_lev_2d".parse().unwrap(), &lib::hyb_recur_lev_2d, tracer));

    res.push(run_func_lev(&v, lev,"hyb_recur_lev_2d_opti".parse().unwrap(), &lib::hyb_recur_lev_2d_opti, tracer));

    if res.windows(2).all(|w| w[0] == w[1]) == false {
        println!("res: {:?}", res);
    } else {
        println!("res: {:?}", res[0]);
    }
}

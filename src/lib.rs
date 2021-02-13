use rand::Rng;

pub fn gen_2d_arr_rand(nb_rows:usize, nb_cols:usize, min: i32, max: i32) -> Vec<Vec<i32>>{
    let mut v= vec![vec![0;nb_cols];nb_rows];
    let mut temp = vec![0; nb_rows];
    for i in 0..nb_rows {
        temp[i] = rand::thread_rng().gen_range(min..max);
    }
    temp.sort();
    temp.reverse();
    for i in 0..nb_rows {
        v[i][0] = temp[i];
    }
    for j in 1..nb_cols {
        if v[0][j-1] == min {
            v[0][j] = min;
        }  else {
            v[0][j] = rand::thread_rng().gen_range(min..v[0][j-1]);
        }
    }
    for i in 1..nb_rows {
        for j in 1..nb_cols {
            let less;
            if v[i][j-1] <= v[i-1][j]{
                less = v[i][j-1];
            } else {
                less = v[i-1][j]
            }
            if less == min {
                v[i][j] = less;
            } else {
                v[i][j] = rand::thread_rng().gen_range(min..less);
            }
        }
    }
    return v;
}

fn seq_iter_1d(v: &[i32]) -> i32{
    v.iter().filter(|&e| *e < 0).count() as i32
}

pub fn seq_iter_2d(v: &[Vec<i32>]) -> i32{
    v.iter().map(| e| seq_iter_1d(e)).sum()
}

fn seq_iter_1d_opti(v: &[i32]) -> i32{
    let nb_ele = v.len();
    if v[0] < 0 {
        nb_ele as i32
    } else if v[nb_ele -1] >= 0 {
        0
    } else {
        v.iter().filter(|&e| *e < 0).count() as i32
    }
}

pub fn seq_iter_2d_opti(v: &[Vec<i32>]) -> i32{
    let nb_rows = v.len();
    let nb_cols = v[0].len();
    if v[0][0] < 0{
        (nb_cols * nb_rows) as i32
    } else if v[nb_rows -1][nb_cols -1] >= 0 {
        0
    } else {
        v.iter().map(|e| seq_iter_1d_opti(e)).sum()
    }
}

fn seq_recur_1d(v_slice: &[i32]) -> i32{
    let n = v_slice.len();
    if n == 1 {
        if v_slice[0] < 0 {
            1
        } else {
            0
        }
    } else {
        let mid = n / 2;
        seq_recur_1d(&v_slice[0..mid]) + seq_recur_1d(&v_slice[mid..n])
    }
}

pub fn seq_recur_2d(v_slice: &[Vec<i32>]) -> i32{
    let n = v_slice.len();
    if n == 1 {
        seq_recur_1d(&v_slice[0])
    } else {
        let mid = n / 2;
        seq_recur_2d(&v_slice[0..mid]) + seq_recur_2d(&v_slice[mid..n])
    }
}

fn seq_recur_1d_opti(v_slice: &[i32]) -> i32{
    let nb_ele = v_slice.len();
    if v_slice[0] < 0 {
        nb_ele as i32
    } else if v_slice[nb_ele -1] >= 0 {
        0
    } else {
        if nb_ele == 1 {
            if v_slice[0] < 0 {
                1
            } else {
                0
            }
        } else {
            let mid = nb_ele / 2;
            seq_recur_1d_opti(&v_slice[0..mid]) + seq_recur_1d_opti(&v_slice[mid..nb_ele])
        }
    }
}

pub fn seq_recur_2d_opti(v_slice: &[Vec<i32>]) -> i32{
    let nb_rows = v_slice.len();
    let nb_cols = v_slice[0].len();
    if v_slice[0][0] < 0{
        (nb_cols * nb_rows) as i32
    } else if v_slice[nb_rows -1][nb_cols -1] >= 0 {
        0
    } else {
        if nb_rows == 1 {
            seq_recur_1d_opti(&v_slice[0])
        } else {
            let mid = nb_rows / 2;
            seq_recur_2d_opti(&v_slice[0..mid]) + seq_recur_2d_opti(&v_slice[mid..nb_rows])
        }
    }
}

fn seq_recur_lev_1d(v_slice: &[i32], lev:i32) -> i32{
    let n = v_slice.len();
    if lev == 0 {
        seq_iter_1d(v_slice)
    } else if n == 1 {
        if v_slice[0] < 0 {
            1
        } else {
            0
        }
    } else {
        let mid = n / 2;
        seq_recur_lev_1d(&v_slice[0..mid], lev - 1) + seq_recur_lev_1d(&v_slice[mid..n], lev - 1)
    }
}

pub fn seq_recur_lev_2d(v_slice: &[Vec<i32>], lev:i32) -> i32{
    let n = v_slice.len();
    if lev == 0 {
        seq_iter_2d(v_slice)
    } else if n == 1 {
        seq_recur_lev_1d(&v_slice[0], lev - 1)
    } else {
        let mid = n / 2;
        seq_recur_lev_2d(&v_slice[0..mid], lev - 1) + seq_recur_lev_2d(&v_slice[mid..n], lev - 1)
    }
}

fn seq_recur_lev_1d_opti(v_slice: &[i32], lev:i32) -> i32{
    let n = v_slice.len();
    if lev == 0 {
        seq_iter_1d_opti(v_slice)
    } else if n == 1 {
        if v_slice[0] < 0 {
            1
        } else {
            0
        }
    } else {
        let mid = n / 2;
        seq_recur_lev_1d_opti(&v_slice[0..mid], lev - 1) + seq_recur_lev_1d_opti(&v_slice[mid..n], lev - 1)
    }
}

pub fn seq_recur_lev_2d_opti(v_slice: &[Vec<i32>], lev:i32) -> i32{
    let n = v_slice.len();
    if lev == 0 {
        seq_iter_2d_opti(v_slice)
    } else if n == 1 {
        seq_recur_lev_1d_opti(&v_slice[0], lev - 1)
    } else {
        let mid = n / 2;
        seq_recur_lev_2d_opti(&v_slice[0..mid], lev - 1) + seq_recur_lev_2d_opti(&v_slice[mid..n], lev - 1)
    }
}

use rayon::prelude::*;
use rayon::join;

fn par_iter_1d(v: &[i32]) -> i32{
    v.par_iter().filter(| &e | *e < 0).count() as i32
}

pub fn par_iter_2d(v: &[Vec<i32>]) -> i32{
    v.par_iter().map(|e| par_iter_1d(e)).sum()
}

fn par_iter_1d_opti(v: &[i32]) -> i32{
    let nb_ele = v.len();
    if v[0] < 0 {
        nb_ele as i32
    } else if v[nb_ele -1] >= 0 {
        0
    } else {
        v.par_iter().filter(|&e| *e < 0).count() as i32
    }
}

pub fn par_iter_2d_opti(v: &[Vec<i32>]) -> i32{
    let nb_rows = v.len();
    let nb_cols = v[0].len();
    if v[0][0] < 0{
        (nb_cols * nb_rows) as i32
    } else if v[nb_rows -1][nb_cols -1] >= 0 {
        0
    } else {
        v.par_iter().map(|e| par_iter_1d_opti(e)).sum()
    }
}

fn par_recur_1d(v_slice: &[i32]) -> i32{
    let n = v_slice.len();
    if n == 1 {
        if v_slice[0] < 0 {
            1
        } else {
            0
        }
    } else {
        let mid = n / 2;
        let mut a:i32 = 0;
        let mut b:i32 = 0;
        join(|| a = par_recur_1d(&v_slice[0..mid]), || b = par_recur_1d(&v_slice[mid..n]));
        a + b
    }
}

pub fn par_recur_2d(v_slice: &[Vec<i32>]) -> i32{
    let n = v_slice.len();
    if n == 1 {
        par_recur_1d(&v_slice[0])
    } else {
        let mid = n / 2;
        let mut a:i32 = 0;
        let mut b:i32 = 0;
        join(|| a = par_recur_2d(&v_slice[0..mid]), || b = par_recur_2d(&v_slice[mid..n]));
        a + b
    }
}

fn par_recur_1d_opti(v_slice: &[i32]) -> i32{
    let nb_ele = v_slice.len();
    if v_slice[0] < 0 {
        nb_ele as i32
    } else if v_slice[nb_ele -1] >= 0 {
        0
    } else {
        if nb_ele == 1 {
            if v_slice[0] < 0 {
                1
            } else {
                0
            }
        } else {
            let mid = nb_ele / 2;
            let mut a: i32 = 0;
            let mut b: i32 = 0;
            join(|| a = par_recur_1d_opti(&v_slice[0..mid]), || b = par_recur_1d_opti(&v_slice[mid..nb_ele]));
            a + b
        }
    }
}

pub fn par_recur_2d_opti(v_slice: &[Vec<i32>]) -> i32{
    let nb_rows = v_slice.len();
    let nb_cols = v_slice[0].len();
    if v_slice[0][0] < 0{
        (nb_cols * nb_rows) as i32
    } else if v_slice[nb_rows -1][nb_cols -1] >= 0 {
        0
    } else {
        if nb_rows == 1 {
            par_recur_1d_opti(&v_slice[0])
        } else {
            let mid = nb_rows / 2;
            let mut a: i32 = 0;
            let mut b: i32 = 0;
            join(|| a = par_recur_2d_opti(&v_slice[0..mid]), || b = par_recur_2d_opti(&v_slice[mid..nb_rows]));
            a + b
        }
    }
}

fn par_recur_lev_1d(v_slice: &[i32], lev:i32) -> i32{
    let n = v_slice.len();
    if lev == 0 {
        par_iter_1d(v_slice)
    } else if n == 1 {
        if v_slice[0] < 0 {
            1
        } else {
            0
        }
    } else {
        let mid = n / 2;
        let mut a:i32 = 0;
        let mut b:i32 = 0;
        join(|| a = par_recur_lev_1d(&v_slice[0..mid], lev - 1), || b = par_recur_lev_1d(&v_slice[mid..n], lev - 1));
        a + b
    }
}

pub fn par_recur_lev_2d(v_slice: &[Vec<i32>], lev:i32) -> i32{
    let n = v_slice.len();
    if lev == 0 {
        par_iter_2d(v_slice)
    } else if n == 1{
        par_recur_lev_1d(&v_slice[0], lev)
    } else {
        let mid = n / 2;
        let mut a:i32 = 0;
        let mut b:i32 = 0;
        join(|| a = par_recur_lev_2d(&v_slice[0..mid], lev - 1), || b = par_recur_lev_2d(&v_slice[mid..n], lev - 1));
        a + b
    }
}

fn par_recur_lev_1d_opti(v_slice: &[i32], lev:i32) -> i32{
    let nb_ele = v_slice.len();
    if v_slice[0] < 0 {
        nb_ele as i32
    } else if v_slice[nb_ele -1] >= 0 {
        0
    } else {
        if lev == 0 {
            par_iter_1d_opti(v_slice)
        } else if nb_ele == 1 {
            if v_slice[0] < 0 {
                1
            } else {
                0
            }
        } else {
            let mid = nb_ele / 2;
            let mut a: i32 = 0;
            let mut b: i32 = 0;
            join(|| a = par_recur_lev_1d_opti(&v_slice[0..mid], lev - 1), || b = par_recur_lev_1d_opti(&v_slice[mid..nb_ele], lev - 1));
            a + b
        }
    }
}

pub fn par_recur_lev_2d_opti(v_slice: &[Vec<i32>], lev:i32) -> i32{
    let nb_rows = v_slice.len();
    let nb_cols = v_slice[0].len();
    if v_slice[0][0] < 0{
        (nb_cols * nb_rows) as i32
    } else if v_slice[nb_rows -1][nb_cols -1] >= 0 {
        0
    } else {
        if lev == 0 {
            par_iter_2d_opti(v_slice)
        } else if nb_rows == 1 {
            par_recur_lev_1d_opti(&v_slice[0], lev)
        } else {
            let mid = nb_rows / 2;
            let mut a: i32 = 0;
            let mut b: i32 = 0;
            join(|| a = par_recur_lev_2d_opti(&v_slice[0..mid], lev - 1), || b = par_recur_lev_2d_opti(&v_slice[mid..nb_rows], lev - 1));
            a + b
        }
    }
}

pub fn hyb_iter_2d(v: &[Vec<i32>]) -> i32{
    v.par_iter().map(| e | seq_iter_1d(e)).sum()
}

pub fn hyb_iter_2d_opti(v: &[Vec<i32>]) -> i32{
    v.par_iter().map(| e | seq_recur_1d_opti(e)).sum()
}

pub fn hyb_recur_2d(v_slice: &[Vec<i32>]) -> i32{
    let n = v_slice.len();
    if n == 1 {
        seq_recur_1d(&v_slice[0])
    } else {
        let mid = n / 2;
        let mut a:i32 = 0;
        let mut b:i32 = 0;
        join(|| a = hyb_recur_2d(&v_slice[0..mid]), || b = hyb_recur_2d(&v_slice[mid..n]));
        a + b
    }
}

pub fn hyb_recur_2d_opti(v_slice: &[Vec<i32>]) -> i32{
    let nb_rows = v_slice.len();
    let nb_cols = v_slice[0].len();
    if v_slice[0][0] < 0{
        (nb_cols * nb_rows) as i32
    } else if v_slice[nb_rows -1][nb_cols -1] >= 0 {
        0
    } else {
        if nb_rows == 1 {
            seq_recur_1d_opti(&v_slice[0])
        } else {
            let mid = nb_rows / 2;
            let mut a: i32 = 0;
            let mut b: i32 = 0;
            join(|| a = hyb_recur_2d_opti(&v_slice[0..mid]), || b = hyb_recur_2d_opti(&v_slice[mid..nb_rows]));
            a + b
        }
    }
}

pub fn hyb_recur_lev_2d(v_slice: &[Vec<i32>], lev:i32) -> i32{
    let n = v_slice.len();
    if lev == 0 {
        hyb_iter_2d(v_slice)
    } else if n == 1 {
        seq_iter_1d(&v_slice[0])
    } else {
        let mid = n / 2;
        let mut a:i32 = 0;
        let mut b:i32 = 0;
        join(|| a = hyb_recur_lev_2d(&v_slice[0..mid], lev - 1), || b = hyb_recur_lev_2d(&v_slice[mid..n], lev - 1));
        a + b
    }
}

pub fn hyb_recur_lev_2d_opti(v_slice: &[Vec<i32>], lev:i32) -> i32{
    let nb_rows = v_slice.len();
    let nb_cols = v_slice[0].len();
    if v_slice[0][0] < 0{
        (nb_cols * nb_rows) as i32
    } else if v_slice[nb_rows -1][nb_cols -1] >= 0 {
        0
    } else {
        if lev == 0 {
            hyb_iter_2d_opti(v_slice)
        } else if nb_rows == 1 {
            seq_recur_1d_opti(&v_slice[0])
        } else {
            let mid = nb_rows / 2;
            let mut a: i32 = 0;
            let mut b: i32 = 0;
            join(|| a = hyb_recur_lev_2d_opti(&v_slice[0..mid], lev - 1), || b = hyb_recur_lev_2d_opti(&v_slice[mid..nb_rows], lev - 1));
            a + b
        }
    }
}
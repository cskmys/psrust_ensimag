fn naive_version_1d(v: &[i32], nb_ele: usize) -> i32{
    let mut cnt = 0;
    for j in 0..nb_ele {
        if v[j] < 0 {
            cnt = cnt + 1;
        }
    }
    return cnt;
}

pub fn naive_version_2d(v: &[Vec<i32>]) -> i32{
    let mut cnt = 0;
    let nb_row = v.len();
    let nb_col = v[0].len();
    for i in 0..nb_row {
        cnt = cnt + naive_version_1d(&v[i], nb_col);
    }
    return cnt;
}

fn recursive_version_1d(v_slice: &[i32]) -> i32{
    let mut cnt = 0;
    let n = v_slice.len();
    if n == 1 {
        if v_slice[0] < 0 {
            cnt = 1;
        }
    } else {
        let mid = n / 2;
        cnt = cnt + recursive_version_1d(&v_slice[0..mid]);
        cnt = cnt + recursive_version_1d(&v_slice[mid..n]);
    }
    return cnt;
}

pub fn recursive_version_2d(v_slice: &[Vec<i32>]) -> i32{
    let mut cnt = 0;
    let n = v_slice.len();
    if n == 1 {
        cnt = cnt + recursive_version_1d(&v_slice[0]);
    } else {
        let mid = n / 2;
        cnt = cnt + recursive_version_2d(&v_slice[0..mid]);
        cnt = cnt + recursive_version_2d(&v_slice[mid..n]);
    }
    return cnt;
}

fn recursive_version_1d_opti(v_slice: &[i32]) -> i32{
    let mut cnt = 0;
    let n = v_slice.len();
    if v_slice[0] < 0 {
        cnt = n as i32;
    } else if v_slice[n-1] >= 0 {
        cnt = 0;
    } else {
        let mid = n / 2;
        cnt = cnt + recursive_version_1d(&v_slice[0..mid]);
        cnt = cnt + recursive_version_1d(&v_slice[mid..n]);
    }
    return cnt;
}

pub fn recursive_version_2d_opti(v_slice: &[Vec<i32>]) -> i32{
    let mut cnt = 0;
    let m = v_slice.len();
    let n = v_slice[0].len();
    if v_slice[0][0] < 0{
        cnt = (n * m) as i32;
    } else if v_slice[m-1][n-1] >= 0 {
        cnt = 0;
    } else {
        if m == 1 {
            cnt = cnt + recursive_version_1d_opti(&v_slice[0]);
            // cnt = cnt + recursive_version_1d_opti(&v_slice[0]);
        } else {
            let mid = m / 2;
            cnt = cnt + recursive_version_2d_opti(&v_slice[0..mid]);
            cnt = cnt + recursive_version_2d_opti(&v_slice[mid..m]);
        }
    }
    return cnt;
}
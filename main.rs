extern crate time;

use std::fs::File;
use std::io::prelude::*;

fn f_min(a:f64, b:f64, c:f64) -> f64 {
    if a < b {
        if a < c {
            return a;
        } else {
            return c;
        }
    }
    else {
        if b < c {
            return b;
        } else {
            return c;
        }
    }
}

fn dtw(x_arr: &[f64], y_arr: &[f64], w: f64) -> f64 {
    assert!(x_arr.len() == y_arr.len(), "arrays are not equal in length");
    assert!(x_arr.len() > 0, "arrays are empty");

    let inf: f64 = std::f64::INFINITY;
    let mat_len = x_arr.len();

    let r: usize = (w * (mat_len as f64)).floor() as usize;

    let mut mat = vec![vec![inf; mat_len+1]; mat_len+1];

    mat[0][0] = 0.0;

    for i in 1..mat_len+1 {
        let left_bound = std::cmp::max(1,(i as i64)-(r as i64));
        let right_bound = std::cmp::min((mat_len+1) as i64, (i+r+1) as i64);
        for j in left_bound..right_bound {
            let iu = i as usize;
            let ju = j as usize;
            let cost = (x_arr[iu - 1] - y_arr[ju - 1]).abs();
            let a = mat[iu-1][ju];
            let b = mat[iu][ju-1];
            let c = mat[iu-1][ju-1];

            mat[iu][ju] = cost + f_min(a,b,c);
        }
    }

    return mat[mat_len][mat_len];

}

// Very specific function that does not generalise, do not use!
fn process_nightly_file(filename: &String) -> Vec<f64> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("error occured reading file to string");
    let lines = contents.split("\r\n").collect::<Vec<&str>>();
    let mut tokens: Vec<&str> = Vec::new();
    for i in 0..lines.len()-1 {
        let a = lines[i].split(",").collect::<Vec<&str>>();
        tokens.push(a[1]);
    }

    let mut x_vec: Vec<f64> = Vec::with_capacity(tokens.len());
    for i in 0..tokens.len() {
        if i % 2 == 0 {
            continue;
        }
        x_vec.push(tokens[i].parse::<f64>().unwrap());
    }

    return x_vec;
}

fn compare_ts(fn1: &String, fn2: &String, w: f64) -> f64 {
    let x_vec = process_nightly_file(fn1);
    let y_vec = process_nightly_file(fn2);

    let result = dtw(x_vec.as_slice(), y_vec.as_slice(), w);
    return result;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fn1 = &args[1];
    let fn2 = &args[2];

    let result = compare_ts(fn1, fn2, 0.1);
    println!("{}", result);

}

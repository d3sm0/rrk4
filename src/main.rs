mod lib;

fn main(){
    let times = vec![0.0, 0.1];
    let y = vec![0.0, 0.1];
    let u:f32 = 0.;
    let mu = 1.;
    let out = lib::rk4(times, y, u,mu);
    println!("{:?}", out);
}


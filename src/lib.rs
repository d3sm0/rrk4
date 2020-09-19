extern crate ndarray;
use ndarray::prelude::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

fn _vdp(_t:f32,x:&Array1<f32>,u:f32,mu:f32) -> Array1<f32>{
    let q = x[0];
    let qdot = x[1];
    let qddot =  mu * qdot * (1.0 - q.powi(2)) - q + u;
    let out =  array![qdot, qddot];
    return out
}

#[pyfunction]
pub fn rk4(times:Vec<f32>, y:Vec<f32>, u:f32, mu:f32)->Vec<f32>{

    let x = Array1::from_vec(y);

    let mut out = Vec::new();

    out.push(x);
    for i in 0..times.len() -1{
        let t = times[i];
        let dt = times[i+1] - t;
        let dt2 = dt/2.0;
        let y0 = &out[i];


        let k1 = _vdp(t, y0, u, mu);
        let k2 = _vdp(t + dt2, &(y0.clone() +  dt2 *   &k1), u, mu);
        let k3 = _vdp(t + dt2, &(y0.clone() +  dt2 *   &k2), u, mu);
        let k4 = _vdp(t + dt2, &(y0.clone() +  dt  *   &k3), u, mu);

        let y1 = y0.clone() + dt /6.0 * (k1 + 2.0 * k2 + 2.0*k3 + k4);

        out.push(y1);
    }
    // let y1 = out[1].clone().to_vec()[0];
    return out[times.len() - 1].clone().to_vec();

}

#[pymodule]
fn rust_vdp(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(rk4))?;
    Ok(())
}

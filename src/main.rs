#![allow(unused_imports)]
mod signle_phase;
use crate::signle_phase::single_phase_line;
use unit_conversions::*;

fn fmt_f64(num: f64, width: usize, precision: usize, exp_pad: usize) -> String {
    let mut num = format!("{:.precision$e}", num, precision = precision);
    // Safe to `unwrap` as `num` is guaranteed to contain `'e'`
    let exp = num.split_off(num.find('e').unwrap());

    let (sign, exp) = if exp.starts_with("e-") {
        ('-', &exp[2..])
    } else {
        ('+', &exp[1..])
    };
    num.push_str(&format!("e{}{:0>pad$}", sign, exp, pad = exp_pad));

    format!("{:>width$}", num, width = width)
}


fn main() {
    // test data
    let inside_id = 13.25;
    let id:f64 = length::inches::to_metres(inside_id);      // pipe inside diameter [inches -> meter]
    let w = 150734.0;       // fluid flow rate [kg/hr]
    let rho = 380.0;        // density [kg/m^3]
    let vis = 0.054 * 0.001;       // viscosity [cP -> kg/(m·s)]
    let e = length::millimetres::to_metres(0.046);       // roughness [mm -> meter]
    let sf = 1.0;         // safety factor

    let mut one_line = single_phase_line::SingleFx::new(w, rho, vis, id, e, sf);

    // output
    println!("Act. ID :  {:.2} inch, equal to {:.8} meters", inside_id, id);
    one_line.velocity();
    println!("Velocity (m/s) : {:.4}", one_line.v);
    one_line.pressure_drop_100();
    println!("Pressure Drop (Kg/cm^2/100m) : {:.6}", one_line.dp100);
    one_line.velocity_head();
    println!("1.0 V.H. (Kg/m/s^2) : {:.4}", one_line.vh);
    one_line.reynold_num();
    println!("Reynold No. [-] : {}", fmt_f64(one_line.nre, 10, 4, 3));

}

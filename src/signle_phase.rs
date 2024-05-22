#![allow(dead_code)]
#![allow(unused_variables)]
pub mod single_phase_line {
    use std::f64::consts::PI;

    pub struct SingleFx {
        // input fields
        w: f64,         // fluid flow rate [kg/hr]
        rho: f64,       // fluid density [kg/m^3]
        mu: f64,        // fluid viscosity [N-sec/m^2] = [Kg/m3/sec]
        id: f64,        // pipe inside diameter [m]
        e: f64,         // pipe roughness [m]
        sf: f64,        // safety factor [-]

        // output fields
        pub v: f64,         // fluid flow velocity [m/s]
        pub nre: f64,       // Reynold number [-]
        pub fdarcy: f64,    // Darcy Friction Factor [-]
        pub dp100: f64,     // pressure drop per 100m
        pub vh: f64,        // velocity head vh = rho * v^2
    }

    impl SingleFx {
        pub fn new(w: f64, rho: f64, mu: f64, id: f64, e: f64, sf: f64) -> SingleFx {
            SingleFx {
                w,
                rho,
                mu,
                id,
                e,
                sf: 1.2,
                v: -999.0,
                nre: -999.0,
                fdarcy: -999.0,
                dp100: -999.0,
                vh: -999.0
            }
        }


        pub fn velocity(&mut self) -> Result<f64, &'static str> {
            let ret: Result<f64, &'static str>;
            if self.rho * self.id != 0.0 {
                self.v = self.w / self.rho / (PI / 4.0 * self.id * self.id ) / 3600.0;
                ret = Ok(self.v);
            }
            else {
                ret = Err("ZeroDivisionError: division by zero caused rho or id");
            }
            return ret;
        }


        pub fn reynold_num(&mut self) -> Result<f64, &'static str> {
            let ret: Result<f64, &'static str>;
            if self.rho * self.id * self.mu != 0.0 {
                self.velocity().expect("velocity calculation error");
                self.nre = self.rho * self.v * self.id / self.mu;
                ret = Ok(self.nre);
            }
            else {
                ret = Err("ZeroDivisionError: division by zero caused rho, id or mu");
            }
            return ret;
        }


        pub fn darcy_friction_factor(&mut self) -> Result<f64, &'static str> {
            let ret: Result<f64, &'static str>;
            if self.rho * self.id * self.mu != 0.0 {
                self.velocity().expect("velocity calculation error");
                self.reynold_num().expect("Reynold Number calculation error");
                if self.nre <= 2300.0 {     // laminar flow
                    self.fdarcy = 64.0 / self.nre;
                } else {    // turbulent flow
                    // Churchill equation
                    let c:f64 = (7.0 / self.nre).powf(0.9) + 0.27 * self.e / self.id;
                    let a:f64 = (2.457 * c.ln()).powf(16.0);
                    let b:f64 = (37530.0 / self.nre).powf(16.0);
                    let term = (8.0 / self.nre).powf(12.0) + 1.0 / (a+b).powf(1.5);
                    let fan = 2.0 * term.powf(1.0/12.0);    // fan: Fanning Friction Factor
                    self.fdarcy = 4.0 * fan;
                }
                ret = Ok(self.fdarcy);
            }
            else {
                ret = Err("ZeroDivisionError: division by zero caused darcy_friction_factor fn");
            }
            return ret;
        }


        pub fn pressure_drop_100(&mut self) -> Result<f64, &'static str> {
            let ret: Result<f64, &'static str>;
            if self.rho * self.id * self.mu != 0.0 {
                let g = 9.80665;
                self.darcy_friction_factor().expect("darcy_friction_factor calculation error");
                self.velocity().expect("velocity calculation error");

                self.dp100 =  self.fdarcy * self.rho * self.v * self.v / (2.0 * g * self.id) / 10000.0 * 100.0 * self.sf;
                ret = Ok(self.dp100);
            }
            else {
                ret = Err("ZeroDivisionError: division by zero caused pressure_drop_100 fn");
            }
            return ret;
        }

    }
}
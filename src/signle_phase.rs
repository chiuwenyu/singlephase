
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
                sf,
                v: -999.0,
                nre: -999.0,
                fdarcy: -999.0,
                dp100: -999.0,
                vh: -999.0
            }
        }

        pub fn velocity(&mut self) -> Result<f64, &'static str> {
            let ret: Result<f64, &'static str>;
            if (self.rho * self.id != 0.0) {
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
            if (self.rho * self.id * self.mu != 0.0) {
                self.velocity();
                self.nre = self.rho * self.v * self.id / self.mu;
                ret = Ok(self.nre);
            }
            else {
                ret = Err("ZeroDivisionError: division by zero caused rho, id or mu");
            }
            return ret;
        }


    }
}
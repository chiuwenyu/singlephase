
pub mod single_phase_line {
    pub struct SingleFx {
        pub rho: f64,       // liquid density [kg/m^3]
        pub mu: f64,        // liquid viscosity [N-sec/m^2] = [Kg/m3/sec]
        pub id: f64,        // pipe inside diameter [m]
        pub w: f64,         // liquid flow rate [kg/hr]
        pub e: f64,         // pipe roughness [m]
        pub fD: f64,        // Darcy Friction Factor [-]
        pub sf: f64,        // safety factor [-]
    }
}
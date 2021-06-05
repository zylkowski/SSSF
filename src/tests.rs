use super::*;
use std::path::{PathBuf};
use assert_approx_eq::assert_approx_eq;

#[test]
fn building() {
    struct DynamicParameters {
        t: f32,
        x: f32,
        y: f32,
        v_x: f32,
        v_y: f32,
    }

    struct EnvironmentParameters {
        dt: f32,
    }

    SimManger::new(
        DynamicParameters {
            t: 0.,
            x: 0.,
            y: 0.,
            v_x: 0.,
            v_y: 0.,
        },
        EnvironmentParameters { dt: 0.1 },
        |dyn_parameteres, environment_parameters| DynamicParameters {
            t: dyn_parameteres.t + environment_parameters.dt,
            x: dyn_parameteres.x,
            y: dyn_parameteres.y,
            v_x: dyn_parameteres.v_x,
            v_y: dyn_parameteres.v_y,
        },
    );
}

#[test]
fn stop_fn_test() {
    struct DynamicParameters {
        t: f32,
    }
    struct EnvironmentParameters {
        dt: f32,
    }

    let mut simulation = SimManger::new(
        DynamicParameters { t: 0. },
        EnvironmentParameters { dt: 1. },
        |dyn_parameteres, environment_parameters| DynamicParameters {
            t: dyn_parameteres.t + environment_parameters.dt,
        },
    );

    simulation.run(|dyn_parameters, _environment_parameters| dyn_parameters.t == 5.);
    assert_approx_eq!(simulation.parameters.last().unwrap().t, 5., 0.1);
}

#[test]
fn to_csv_test() {
    struct DynamicParameters {
        t: i32,
        x: i32,
        y: i32,
    }
    impl ToCSV for DynamicParameters {
        fn get_header() -> String {
            String::from("t,x,y")
        }
        fn get_row(&self) -> String {
            format!("{},{},{}", self.t, self.x, self.y)
        }
    }

    let param = DynamicParameters { t: 0, x: 1, y: 2 };

    let header = DynamicParameters::get_header();
    assert_eq!("t,x,y", header);

    let row = param.get_row();
    assert_eq!("0,1,2", row);
}

#[test]
fn file_output() {
    struct DynamicParameters {
        t: f32,
        x: i32,
        y: i32,
    }
    impl ToCSV for DynamicParameters {
        fn get_header() -> String {
            String::from("t,x,y")
        }
        fn get_row(&self) -> String {
            format!("{},{},{}", self.t, self.x, self.y)
        }
    }
    struct EnvironmentParameters {
        dt: f32,
    }

    let mut _simulation = SimManger::new(
        DynamicParameters { t: 0., x: 4, y: 7 },
        EnvironmentParameters { dt: 1. },
        |dyn_parameteres, environment_parameters| DynamicParameters {
            t: dyn_parameteres.t + environment_parameters.dt,
            x: dyn_parameteres.x + 1,
            y: dyn_parameteres.y + 3,
        },
    ).to_file(PathBuf::from(r"output.csv"))
    .run_with_save(|dyn_parameters, _environment_parameters| dyn_parameters.t == 5.);
}

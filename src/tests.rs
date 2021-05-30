use super::*;
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
        g: f32,
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
        EnvironmentParameters { g: 9.87, dt: 0.1 },
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

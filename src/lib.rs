pub struct SimManger<DynamicParameters, EnvironmentParameters, StepFn, StopFn>
where
    StepFn: FnMut(&DynamicParameters, &mut EnvironmentParameters) -> DynamicParameters,
    StopFn: FnMut(&DynamicParameters, &EnvironmentParameters) -> bool,
{
    parameters: Vec<DynamicParameters>,
    environment_parameters: EnvironmentParameters,
    step_fn: StepFn,
    stop_fn: StopFn,
}

impl<DynamicParameters, EnvironmentParameters, StepFn, StopFn>
    SimManger<DynamicParameters, EnvironmentParameters, StepFn, StopFn>
where
    StepFn: FnMut(&DynamicParameters, &mut EnvironmentParameters) -> DynamicParameters,
    StopFn: FnMut(&DynamicParameters, &EnvironmentParameters) -> bool,
{
    fn new(
        initial_conditions: DynamicParameters,
        environment_parameters: EnvironmentParameters,
        step_fn: StepFn,
        stop_fn: StopFn,
    ) -> Self {
        SimManger {
            parameters: vec![initial_conditions],
            environment_parameters,
            step_fn,
            stop_fn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            |dyn_paramteres, environment_parameters| DynamicParameters {
                t: dyn_paramteres.t + environment_parameters.dt,
                x: dyn_paramteres.x,
                y: dyn_paramteres.y,
                v_x: dyn_paramteres.v_x,
                v_y: dyn_paramteres.v_y,
            },
            |dyn_paramteres, environment_parameters| dyn_paramteres.t == 100,
        );
    }
}

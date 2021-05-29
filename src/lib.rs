pub struct SimManger<DynamicParameters, EnvironmentParameters, StepFn>
where
    StepFn: FnMut(&DynamicParameters, &mut EnvironmentParameters) -> DynamicParameters,
{
    parameters: Vec<DynamicParameters>,
    environment_parameters: EnvironmentParameters,
    step_fn: StepFn,
}

impl<DynamicParameters, EnvironmentParameters, StepFn>
    SimManger<DynamicParameters, EnvironmentParameters, StepFn>
where
    StepFn: FnMut(&DynamicParameters, &mut EnvironmentParameters) -> DynamicParameters,
{
    fn new(
        initial_conditions: DynamicParameters,
        environment_parameters: EnvironmentParameters,
        step_fn: StepFn,
    ) -> Self {
        SimManger {
            parameters: vec![initial_conditions],
            environment_parameters,
            step_fn,
        }
    }

    fn run<StopFn>(&mut self, stop_fn: StopFn)
    where
        StopFn: Fn(&DynamicParameters, &EnvironmentParameters) -> bool,
    {
        loop {
            let new_parameters = (self.step_fn)(
                self.parameters.last().unwrap(),
                &mut self.environment_parameters,
            );
            self.parameters.push(new_parameters);

            if stop_fn(
                self.parameters.last().unwrap(),
                &self.environment_parameters,
            ) {
                break;
            }
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
    fn running() {
        struct DynamicParameters {
            t: f32,
        }
        struct EnvironmentParameters {
            dt: f32,
        }

        SimManger::new(
            DynamicParameters {
                t: 0.,
            },
            EnvironmentParameters { dt: 1. },
            |dyn_parameteres, environment_parameters|{
                DynamicParameters {
                    t: dyn_parameteres.t + environment_parameters.dt,
                }
            } 
        ).run(|dyn_parameters, environment_parameters| dyn_parameters.t == 5.);
    }
}

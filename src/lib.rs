pub struct SimManger<DynamicParameters, EnvironmentParameters> {
    parameters: Vec<DynamicParameters>,
    environment_parameters: EnvironmentParameters,
}
impl<DynamicParameters, EnvironmentParameters> SimManger<DynamicParameters, EnvironmentParameters> {
    fn with_initial_conditions(
        initial_conditions: DynamicParameters,
        environment_parameters: EnvironmentParameters,
    ) -> Self {
        SimManger {
            parameters: vec![initial_conditions],
            environment_parameters,
        }
    }

    fn with_step<StepF>(&self, step_f: StepF)
    where
        StepF: FnMut(&DynamicParameters, &mut EnvironmentParameters) -> DynamicParameters,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building() {
        struct DynamicParameters {
            x: f32,
            y: f32,
            v_x: f32,
            v_y: f32,
        }
        struct EnvironmentParameters {
            g: f32,
        }

        SimManger::with_initial_conditions(
            DynamicParameters {
                x: 0.,
                y: 0.,
                v_x: 0.,
                v_y: 0.,
            },
            EnvironmentParameters { g: 9.87 },
        );
    }
}

pub struct SimManger<DynamicParameters, EnvironmentParameters, StepFn>
where
    StepFn: FnMut(&DynamicParameters, &mut EnvironmentParameters) -> DynamicParameters,
{
    // parameters that change while simulation is running and that we want to keep track of, eg. particle's position
    pub(crate) parameters: Vec<DynamicParameters>,
    // parameters that are constant throughout whole simulation or those we don't need to track, eg. gravitational constant
    pub(crate) environment_parameters: EnvironmentParameters,
    pub(crate) step_fn: StepFn,
}

impl<DynamicParameters, EnvironmentParameters, StepFn>
    SimManger<DynamicParameters, EnvironmentParameters, StepFn>
where
    StepFn: FnMut(&DynamicParameters, &mut EnvironmentParameters) -> DynamicParameters,
{
    pub fn new(
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

    pub fn run<StopFn>(&mut self, stop_fn: StopFn)
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

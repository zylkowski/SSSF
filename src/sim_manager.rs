use std::path::{Path, PathBuf};

pub trait ToCSV{
    fn get_header() -> String;
    fn get_row(&self) -> String; 
}

pub struct SimManger<DynamicParameters, EnvironmentParameters, StepFn>
where
    StepFn: FnMut(&DynamicParameters, &mut EnvironmentParameters) -> DynamicParameters,
{
    // parameters that change while simulation is running and that we want to keep track of, eg. particle's position
    pub(crate) parameters: Vec<DynamicParameters>,
    // parameters that are constant throughout whole simulation or those we don't need to track, eg. gravitational constant
    pub(crate) environment_parameters: EnvironmentParameters,
    pub(crate) step_fn: StepFn,
    pub(crate) output_file: Option<PathBuf>
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
            output_file: None
        }
    }

    pub fn run<StopFn>(&mut self, stop_fn: StopFn) -> &mut Self
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
        self
    }
}

impl<DynamicParameters, EnvironmentParameters, StepFn>
    SimManger<DynamicParameters, EnvironmentParameters, StepFn>
where
    DynamicParameters: ToCSV,
    StepFn: FnMut(&DynamicParameters, &mut EnvironmentParameters) -> DynamicParameters,
{
    pub fn to_file(&mut self, path: PathBuf) -> &mut Self {
        self.output_file = Some(path);
        self
    }

    pub fn run_with_save<StopFn>(&mut self, stop_fn: StopFn) -> &mut Self
    where
        StopFn: Fn(&DynamicParameters, &EnvironmentParameters) -> bool,
    {
        self.run(stop_fn);
        self.output_to_file();
        self
    }

    fn output_to_file(&self) {
        todo!()
    }
}
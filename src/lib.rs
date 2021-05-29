
pub struct SimManger{

}
impl SimManger{
    fn with_initial_conditions(initial_conditions: DynConditions) -> &Self {
        todo!()
    } 

    fn with_step(step_f: FnMut(&DynConditions,&mut StaticConditions) -> DynConditions) {
        todo!()
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_1(){

    }
}
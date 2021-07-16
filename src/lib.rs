//! # SSSF - Super Simple Simulation Framework
//!
//! It's crate that lets you write Rust physics simulations without boring and repetetive boilerplate. I created this for my personal usage because of my Uni classes in which we wrote lots of physical simulations but now I want to share because I think it's pretty elegant and may be useful for people in similar situations. It's well suited for small physical simulations but I can see it usefull in bigger projects too, although in bigger projects it may be a good idea to create something tailored for your problem.
//! 
//! ## Usage
//!
//! Let's say you would like to build simulation of body moving with constant velocity on x-y plane. Then you need to create struct that holds information about that body in for each timestep:
//!
//! ```rust
//! BodyParameters{
//!   t: f32          // time,
//!   x: f32,
//!   y: f32,
//!   v_x: f32,
//!   v_y: f32,
//! }
//! ```
//!
//! These parameters will be stored as history after EACH timestep, that's why we added `t` field in `BodyParameters` to keep track of history of position (`x`,`y`) and velocity (`V_x`,`v_y`) changing in time.
//! Now that we took care of parameters that we want to save history of we can move to those parameters that are either constant throughout whole simulation or those that we don't need/want to save history of - for example we wouldn't like to save whole grid for Conway's Game of Life for each timestep - we would hold it in parameters without history. So coming back to the example of moving body. We can think of those parameters as environment that we don't want to track.
//! ```rust
//! EnvironmentParameters{
//!   dt: f32
//! }
//! ```
//! For purpose of this example we don't need to store there anything complicated so lets just store `dt` - delta time.
//! After that we can start our simulation providing `BodyParameters`, `EnvironmentParameters` and step function for simulation like this:
//! ```rust
//! let mut simulation = SimManger::new(
//!     BodyParameters { t: 0., x: 0., y:0., v_x:1., v_y:1. },                      // Initial parameters for the body
//!     EnvironmentParameters { dt: 0.1 },
//!     |body_parameteres, environment_parameters| BodyParameters {                 // Defining step function
//!         t: body_parameters.t + environment_parameters.dt,
//!         x: body_parameters.x + environment_parameters.dt * body_parameters.v_x,
//!         y: body_parameters.y + environment_parameters.dt * body_parameters.v_y,
//!         v_x: body_parameters.v_x,
//!         v_y: body_parameters.v_y,
//!     },
//! );
//! ```
//! As shown above you need to provide `step_fn: FnMut(&BodyParameters, &mut EnvironmentParameters) ->BodyParameters` it has to return `BodyParameters` after this simulation step, they will be automatically saved.
//! Now in order to run the simulation you call `.run(stop_fn)` where `stop_fn: Fn(&DynamicParameters, &EnvironmentParameters) -> bool` it's simple function that is checked after each timestep. If it's true, simulation stops.
//! Eg.  
//! If you want to stop simulation when `t = 5 s` you call:  
//! ```simulation.run(|body_parameters, _environment_parameters| body_parameters.t == 5.);```  
//! If you want to stop simulation when `x = 20 m` you call:  
//! ```simulation.run(|body_parameters, _environment_parameters| body_parameters.x == 20.);```
//! 
//! ## Writing to file
//! It's very useful to save simulation history to file to analyze it further, make some beautiful plots etc. In this case you would like to implement `ToCSV` trait for `BodyParameters`. I didn't want to use `serde` for serialization because I want this framework to be super lightweight and it's not really necessary.
//! You can implement `ToCSV` like this:
//! ```rust
//! impl ToCSV for BodyParameters {
//!     fn get_header() -> String {
//!         String::from("t,x,y")             // Header for .csv file (don't contain "\n" in this string, it's added automatically)
//!     }
//!     fn get_row(&self) -> String {
//!         format!("{},{},{}", self.t, self.x, self.y)     // Contents of .csv file (don't contain "\n" in this string, it's added automatically)
//!     }
//! }
//! ```
//! Then insted of calling `.run(stop_fn)` you have to call like this:
//! ```rust
//! simulation.to_file(PathBuf::from(r"output.csv"))
//!           .run_with_save(stop_fn);
//! ```
//! 

mod sim_manager;
pub use sim_manager::*;

#[cfg(test)]
mod tests;

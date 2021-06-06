# SSSF - Super Simple Simulation Framework
It's crate that lets you write Rust physics simulations without boring and repetetive boilerplate. I created this for my personal usage because of my Uni classes in which we wrote lots of physical simulations but now I want to share because I think it's pretty elegant and may be usefull for people in similar situations. It's well suited for small physical simulations but I can see it usefull in bigger projects too, although in bigger projects it may be a good idea to create something tailored for your problem.

## Usage
Let's say you would like to build simulation of moving body. Then you need to create struct that holds information about that body in for each timestep:
```rust
BodyParameters{
  t: f32          // time,
  x: f32,
  y: f32,
  v_x: f32,
  v_y: f32,
}
```
These parameters will be stored as history after EACH timestep, that's why we added `t` field in `BodyParameters` to keep track of history of position (`x`,`y`) and velocity (`V_x`,`v_y`) changing in time.
Now that we took care of parameters that we want to save history of we can move to those parameters that are either constant throughout whole simulation or those that we don't need/want to save history of - for example we wouldn't like to save whole grid for Conway's Game of Life for each timestep - we would hold it in parameters without history. So coming back to the example of moving body. We can think of those parameters as environment that we don't want to track.
```rust
EnvironmentParameters{
  dt: f32
}
```
For purpose of this example we don't need to store there anything complicated so lets just store `dt` - delta time.

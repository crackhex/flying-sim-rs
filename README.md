  # Super Mario 64 Flying Simulator Rust
This is a WIP flying simulator for Super Mario 64, built in rust. Currently, there is very little functionality.


## Goals:
- Write safe, fast, logically equivalent code to Super Mario 64's flying action updates in rust.
- Integrate neural network training to brute-force flying paths
- Seamlessly and efficiently run the simulation on the GPU

The first two goals are in the works, but are not expected to be the main roadblock for this project. **The last goal will be the most difficult.**

## Possible Future Goals:

- Implement other actions, like swimming and air_with_turn to simulate
- Implement collision detection for surface triangles

As this is simply a hobby project, the implementation of surface triangles is not to be expected, however _could_ happen later down the line, assuming motivation to do such is strong enough. Surface collision is jank enough as is, rewriting it would require extra time and effort.

# Licence: 
This is licenced with GPL V3.0. Anyone may feel free to distribute and modify according to the licence. 

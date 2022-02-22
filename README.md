# Cave Generator using Bevy
> This repository contains the code for one of my blog posts ([link](https://florentcollin.github.io/2022/02/20/cave-generator/)).
> The goal was to create a Cave Generator based on the Bevy Game Engine and Cellular Automata. The cave is simply represented as a grid of white and black squares that represent the cave. 
> An overview of what is built in the blog is available below.

![Cave Generator preview](https://florentcollin.github.io/2022/02/20/cave-generator/cave-generator-final.gif)

# Run
```sh
cargo run --release
# Eventually, if you want to dynamically build the executable
cargo run --features bevy/dynamic --release
```
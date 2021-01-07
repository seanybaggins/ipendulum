# Introduction
This directory contains all of the code needed to implement the inverted pendulum using the `STM32F303VCT6` discovery board.

Make sure to open the workspace within the provided devcontainer at the directory of this `README.md`. Instructions for how to open a dev container can be found [here](../README.md).

The provided dev container comes with a dubug server, a debugger, and a tool for flashing the microcontroller.

To flash and run the application run
```
cargo embed
```

To debug the microcontroller first start the gdb server by running 
```
openocd
```

Attach to the gdbserver and load the application with breakpoints simply by running 
```
cargo run
```

You can run unit tests on the host machine by running
```
test
```
in the bash shell

## References
[stm32f3 reference manual](http://www.st.com/resource/en/reference_manual/dm00043574.pdf)
[stm32f3 user manual](http://www.st.com/resource/en/user_manual/dm00063382.pdf)
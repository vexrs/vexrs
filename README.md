# Vexrs -- Culpeper Experimental Robotics Operating System

Vexrs is an experimental operating system being developed to allow robotics teams to take full advantage of the VEX V5 hardware. Vexrs is designed to be lightweight and offer 


## Disclaimer
This crate generates bindings to libv5rt. To compile this crate, set the LIBV5_PATH environment variable to the path of your Vex Code installation. The crate will copy the contents to the ~/.v5 folder and create a file called .v5init. Once this is done, you do not need to worry about setting the environment variable again. Please note that the entirety of libv5rt is property of Innovation First Inc. We are not affiliated to Innovation First Inc. We do not officially support generating these bindings from Vex Code. This program generated bindings at compile time and keeps all generated files on the user's machine. As such, this program contains no code or files from any Vex Software.

This software is not endorsed, affiliated, or sanctioned by Innovation First, Inc. The entirety of the libv5rt library belongs to Innovation First, Inc. VEX and VEX Robotics are trademarks of Innovation First, Inc. If they so request, this project will be taken offline.

Utilizing this software to generate bindings to libv5rt should be done at the user's discretion.

The code contained in this branch is subject to the MIT license. The MIT license contained in this branch may not cover libraries used by this crate and does not cover libv5rt which is property of VEX Robotics and subject to their terms.


## About


For optimizations settings, see Cargo.toml

LTO may break floating point operations. LTO can be disabled at the cost of file size in Cargo.toml. Enabling the build-std-feature `compiler-builtins-mangled-names` fixes this.

## Usage

This project by default uses an internal program to upload and execute the program on the v5. First it objcopy's the elf file into a flat binary, and then uploads it to the brain. PROS cli can also be used to do this as shown in Queen's Robotics [vex_rt](https://gitlab.com/qvex/vex-rt)
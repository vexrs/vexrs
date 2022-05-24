# CEROS -- Culpeper Experimental Robotics Operating System

CEROS is an experimental operating system being developed to allow robotics teams to take full advantage of the VEX V5 hardware.


## Disclaimer
    This crate generates bindings to libv5rt. To compile this crate, set the LIBV5_PATH environment variable to the path of your Vex Code installation. The crate will copy the contents to the ~/.v5 folder and create a file called .v5init. Once this is done, you do not need to worry about setting the environment variable again. Please note that the entirety of libv5rt is property of Innovation First Inc. We are not affiliated to Innovation First Inc. We do not officially support generating these bindings from Vex Code. This program generated bindings at compile time and keeps all generated files on the user's machine. As such, this program contains no code or files from any Vex Software.

    This software is not endorsed, affiliated, or sanctioned by Innovation First, Inc. The entirety of the libv5rt library belongs to Innovation First, Inc. VEX and VEX Robotics are trademarks of Innovation First, Inc. If they so request, this project will be taken offline.

    This utilizing this software to generate bindings to libv5rt should be done at the user's discretion.

    The code contained in this branch is subject to the MIT license. The MIT license contained in this branch may not cover libraries used by this crate and does not cover libv5rt which is property of VEX Robotics and subject to their terms.


## About

This branch contains only the CEROS runtime, a lightweight green-threading runtime designed to be simple to use and understand.

This is the minimal implementation of such a runtime and should be used only as reference for implementing a larger runtime. When reading the code and using the software, a few things must be noted:
- The runtime uses an UnsafeCell. This is bad and it may be possible to circumvent this. However, due to the nature of green-threaded runtimes it is equally possible that any other method is impossible. If you happen to know a fix please file an issue, or even better submit a pull request.
- This crate has only a few runtime dependencies. The newlib_alloc dependency can be removed if you are using your own allocator, but this is not recommended. lazy_static can be removed if you somehow find another way to have the runtime as a global singleton or if rust eventually adopts lazy_static as a builtin feature. Libc can be removed if you modify this crate to no longer depend on libv5rt.
- The only references to libv5rt are in the panic handler (printing to screen) and in main.rs (delays and printing to screen).
- The runtime code is not dependent on libv5rt nor does it depend on using a v5 brain. However, it *does* depend on the armv7-a target.


Compilation Stats:
- Dev with libv5rt: 2.2 MB elf, 106.5 kB bin
- Dev without libv5rt: 2.2 MB elf, 87.7 kB bin
- Release with libv5rt: 81.6 kB elf, 15.2 kB bin
- Release without libv5rt: 75.1 kB elf, 8.8 kB bin

For optimizations settings, see Cargo.toml

LTO may break floating point operations. LTO can be disabled at the cost of file size in Cargo.toml

## Usage

This project by default uses an internal program to upload and execute the program on the v5. First it objcopy's the elf file into a flat binary, and then uploads it to the brain. PROS cli can also be used to do this as shown in Queen's Robotics [vex_rt](https://gitlab.com/qvex/vex-rt)
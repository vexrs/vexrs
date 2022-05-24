# CEROS -- Culpeper Experimental Robotics Operating System

CEROS is an experimental operating system being developed to allow robotics teams to take full advantage of the VEX V5 hardware.

This branch contains only the CEROS runtime, a lightweight green-threading runtime designed to be simple to use and understand.

This is the minimal implementation of such a runtime and should be used only as reference for implementing a larger runtime. When reading the code and using the software, a few things must be noted:
- The runtime uses an UnsafeCell. This is bad and it may be possible to circumvent this. However, due to the nature of green-threaded runtimes it is equally possible that any other method is impossible. If you happen to know a fix please file an issue, or even better submit a pull request.
- This crate generates bindings to the vex libv5rt. To compile this crate, set the LIBV5_PATH environment variable to the path of your Vex Code installation. The crate will copy the contents to the ~/.v5 folder and create a file called .v5init. Once this is done, you do not need to worry about setting the environment variable again. Please note that the entirety of libv5rt is property of Innovation First Inc. We are not affiliated to Innovation First Inc. We do not officially support generating these bindings from Vex Code. This program generated bindings at compile time and keeps all generated files on the user's machine. As such, this program contains no code or files from any Vex Software.

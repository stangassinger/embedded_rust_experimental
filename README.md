$:   openocd -f openocd.cfg & \n
$:   cargo run

### alternativly you can run it with semihosting mode in one line:

xfce4-terminal --command 'sudo openocd -f openocd.cfg'  --hold & gdb-multiarch  target/thumbv7em-none-eabihf/debug/embedded_rust_experiment -q -x openocd.gdb

### to do:
### implement os with template of this:
https://github.com/jserv/mini-arm-os

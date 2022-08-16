## What is this?

I wanted a script that I could use to manage my servers during a power outage using ipmi.

## TODO

- [x] Create systemd service for this program
- [ ] Add support for sending commands to other hosts over ip

## Building and running
- Install rust
- Install arm-linux-gnueabihf-gcc for raspberry pi cross-compile
- Run `./deploy.sh` after editing editing the default values
- SSH into the target machine and run the program after installing the .env file
Purpose
=======
I am using a Raspberry Pi 4 with Argon 40 Fan Hat running Debian. The provided software does not work (at least not on Debian). As far as I understand the Cython GPIO library (DEB and Pip (RPi.GPIO 0.7.0, smbus 1.1.post2)) used does not recognize it is running on a Pi. I looked into Argon40s code and wrote my own version in Rust.

Differences
===========
No config file used. I linearly set the fan speed. 30 C -> 1% (min), 65 C -> 100%.

Code state
==========
Work in progress. Sensor reading and fan setting works. Button not implemented.
I'd like to have some convenience later.

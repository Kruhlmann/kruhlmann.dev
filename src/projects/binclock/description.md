    binclock <output_device> <input_device>

Default sense device on a raspberry pi is typically `/dev/fb0` or `/dev/fb1`. The input device is one of the devices listen under `/dev/input/event*`. See [stick.py:75](https://github.com/astro-pi/python-sense-hat/blob/9a37f0923ce8dbde69514c3b8d58d30de01c9ee7/sense_hat/stick.py#L75) for details on how this is located.

## Installation

    $ sudo make install


# Calliope Board support crate

This project should become a board support crate for the
Calliope mini.

As of now, the code is expected to work with a Calliope mini 1.0
due to hardware availability.

The Calliope mini 1.0 is very similar to the Microbit v1.3 and uses the
same Nordic nRF51822 processor.
Some code, especially for the 25 LEDs work on both boards.

Therefore, we will take some code from the microbit board support
crate and examples from the rust discovery book.

Warning: I'm still learning embedded Rust and HAL design, so none of
this looks good yet!

# ToDo

* Improve many things
* Demos are bypassing the board support crates functions
* Bring microphone to operation
* Bring buttons to operation
* Bring analog input to operation
* Bring microphone to operation
* Bring touch input to operation
* Bring radio to operation
* Bring microphone to operation


# References

References:

* https://calliope.cc/
* https://crates.io/crates/nrf51-hal
* https://crates.io/crates/microbit
* https://docs.rust-embedded.org/discovery/microbit/

Developer Setup:

* https://docs.rust-embedded.org/discovery/microbit/03-setup/index.html

Dependencies:

* https://crates.io/crates/nrf51-hal
* https://docs.rs/nrf51-hal/latest/nrf51_hal/

* https://crates.io/crates/embedded-hal
* https://docs.rs/embedded-hal/latest/embedded_hal/
* https://github.com/rust-embedded/embedded-hal


Furhter links:

https://github.com/rust-embedded/cortex-m-quickstart


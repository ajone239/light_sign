Light Sign
==========
During the COVID times, I have been relegated to turning my room in my apartment into an office.
This sign was designed to deter roomates from knocking on my door when I am working.
This was written in Rust, a language for which I have very little experience and alot of love.
Hope, you like it.

Overview
--------

Components
----------

If you wish to reimplement this, listed below are the parts I used and why.
I take no responsibility for damaged electronics or fires or anything really <3.

- [Raspberry Pi Zero W](https://www.raspberrypi.org/products/raspberry-pi-zero-w/)
This will host the web server and parse out the query from the web request.
Any Raspberry Pi will do; this one just made the most sense to me.

*NOTE:* Cross compilation will be much easier if you don't use the Raspberry Pi Zero W.
This was a huge speed bump, but below will link a guide I used for cross compiling.

- [Arduino Nano](https://store.arduino.cc/usa/arduino-nano)
The Arduino is responsible for displaying the requested string on the LED Matrix.
I'm sure you could do this with the Raspberry Pi, but I didn't so yeah.
Again, any Arduino could be used.
I had this on hand and the task isn't that demanding.

- [Addressable 8x32 LED Matrix](https://www.amazon.com/dp/B01DC0IPVU)
Any light matrix will do here so long as it is made up of *WS2812B* addressable leds.
This will have to be diffused as it is super bright.

- [5V 5A PSU](https://www.amazon.com/gp/product/B078RT3ZPS)
This gives us power.
Make sure you have enough.
And also not too much.

Install
-------

### Arduino

### Raspberry Pi Config

### Rust

### Cross Compiling

### Hardware Setup

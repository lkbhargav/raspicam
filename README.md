# raspicam
Tiny Rust library for playing with RaspberryPi's camera.
This library is supposed to click an image from the RaspberryPi's camera and stores the image in the user's specified path.
As of now, this library works only for clicking images from the camera.
We thrive for the best and want you to contribute towards a better Project. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for giving your valuable feedback and contributions.

### Usage

*In your Cargo.toml*

```
[dependencies]
raspicam = "0.1.0"
```
### Cross-compilation build for Raspberry Pi

Run `cargo build --target=armv7-unknown-linux-gnueabihf` to get a cross compiled binary in `/target/armv7-unknown-linux-gnueabihf/debug/raspicam`

## Running the binary

```
./raspicam
```

#### Example

+ [Simple Example](https://github.com/pawanbisht62/raspicam/blob/master/src/bin/click_image.rs)
# Platform agnostic driver written using embedded-hal traits to interface with the ADC128S022 ADC's

## The device

Datasheet:
- [ADC128S022](https://www.ti.com/lit/ds/symlink/adc128s022.pdf?ts=1693829345688&ref_url=https%253A%252F%252Fwww.ti.com%252Fproduct%252Fde-de%252FADC128S022)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

```rust
use adc128s022::{Adc128s022, Channel};
use embedded_hal::spi::blocking::ExclusiveDevice;
use linux_embedded_hal::{Spidev, SysfsPin};

fn main() {
    let spi = Spidev::open("/dev/spidev0.0").unwrap();
    let chip_select = SysfsPin::new(25);
    let dev = ExclusiveDevice::new(spi, chip_select);
    let mut adc = Adc128s022::new(dev);
    adc.read_channel(Channel::Ch1).unwrap();

    // Get device back
    let _dev = adc.destroy();
}
```

## Support

For questions, issues, feature requests like compatibility with similar devices
and other changes, please file an
[issue in the github project](https://github.com/schphil/adc128s022-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

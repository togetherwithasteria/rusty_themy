<div align="center">

# 🦀🦀 Rusty Themy 🎨🎨
*Obtain the current color theme in your system!*

</div>

```sh
$ cargo add rusty_themy
```
## Getting the current theme on Linux
It's easy! Just enable the `gtk` feature and use 
```rs
rusty_themy::gtk::current::current()
```
You can see the example here!
```sh
cargo run --example current_theme --features gtk
```
The output will be a JSON dump of the colors that I hope, get applied by GTK for its applications.
```rust
(
    {
        ...
        "dialog_bg_color": RGBA(
            RGBA {
                red: 247,
                green: 219,
                blue: 239,
                alpha: 255,
            },
        ),
        "light_4": RGBA(
            RGBA {
                red: 192,
                green: 191,
                blue: 188,
                alpha: 255,
            },
        ),
        "red_4": RGBA(
            RGBA {
                red: 192,
                green: 28,
                blue: 40,
                alpha: 255,
            },
        ),
        ...
    },
    [ /* any errors go here */ ],
)
```
Custom colors that are defined by you will also be included! Not just the ones from GTK or Libadwaita.

Are you confused on what color to consume? Good place to start would be to visit the [Libadwaita docs on Named Colors](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.2/named-colors.html) or play with the [Gradience](https://github.com/GradienceTeam/Gradience) app!

## Getting color themes on other platforms
Unfortunately, they are not supported yet.<br/>
But... feel free to make a PR!
## 📜 License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### 💁 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
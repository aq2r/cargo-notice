# cargo-notice

> This contents of this README were translated from Japanese to English using Google Translate.

It will check whether libraries other than the specified licenses are included, and will output the licenses of the libraries used in a markdown file.

## Install

```sh
$ cargo install --git https://github.com/aq2r/cargo-notice --locked
```

## How to use

### init

```sh
# After this, you will be asked which license to
# allow based on the licenses of the dependent libraries, so select one.
$ cargo notice init
```

### check

```sh
# License names included in the allowed list will be displayed in green, and those not included will be displayed in red.
# If the conditions are not met, an error will occur.
$ cargo notice check
```

```sh
# Execution result:
aho-corasick v1.1.3: Unlicense OR MIT
anstream v0.6.20: MIT OR Apache-2.0
anstyle v1.0.11: MIT OR Apache-2.0
anstyle-parse v0.2.7: MIT OR Apache-2.0
anstyle-query v1.1.4: MIT OR Apache-2.0
anstyle-wincon v3.0.10: MIT OR Apache-2.0
anyhow v1.0.99: MIT OR Apache-2.0
autocfg v1.5.0: Apache-2.0 OR MIT
bitflags v2.9.3: MIT OR Apache-2.0
camino v1.1.11: MIT OR Apache-2.0
cargo-platform v0.3.0: MIT OR Apache-2.0
cargo-util-schemas v0.8.2: MIT OR Apache-2.0
cargo_metadata v0.22.0: MIT
cfg-if v1.0.3: MIT OR Apache-2.0
clap v4.5.45: MIT OR Apache-2.0
clap_builder v4.5.44: MIT OR Apache-2.0
clap_derive v4.5.45: MIT OR Apache-2.0
clap_lex v0.7.5: MIT OR Apache-2.0
...
```

### generate

```sh
$ cargo notice generate

# When outputting to a file
$ cargo notice generate > ThirdPartyLicense.md
# or
$ cargo notice generate > NOTICE.md
```

[Here](./sample/GenerateSample.md) is a sample of the output.

### Manual configuration

Some libraries may not have a license that can be read mechanically.

In these cases, you can manually indicate the license of the library.

For example, add the following to the `[license-manual]` section:

`cargo-notice.toml`:

```toml
allow-license = [
    "MIT",
    "Apache-2.0",
]

[license-manual]
ring = "MIT AND ISC AND OpenSSL"
```

## Note

Please note that this program does not necessarily automatically check and collect all licenses. Recommend that you check the results manually.

## Differences from other tools

- [**cargo-license**](https://github.com/onur/cargo-license)

  cargo-license displays a list of dependent libraries.
  cargo-notice checks licenses and outputs a list.

- [**cargo-about**](https://github.com/EmbarkStudios/cargo-about)

  cargo-about outputs information about dependent libraries in HTML, while cargo-notice outputs it in Markdown.

- [**cargo-bundle-licenses**](https://github.com/sstadick/cargo-bundle-licenses)

  cargo-bundle-license outputs information about dependent libraries in json, yaml, toml, while cargo-notice outputs it in Markdown.

  And, I used the cargo-bundle-licenses README as a reference for writing this README.

- [**cargo-deny**](https://github.com/EmbarkStudios/cargo-deny)

  If you want to do more complicated checking and check various aspects other than just the license, cargo-deny is probably more suitable.

  If you want to simply and easily check the licenses of dependent libraries, cargo-notice is probably more suitable.

## LICENSE

Licensed either of [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT) License at your option.

# xkpasswd-rs

XKCD password generator, written in Rust with supports for Wasm in mind.

## Acknowledgement

This project is highly inspired by:

* XKCD's [Password Strength](https://xkcd.com/936/) comic,
* [xkpasswd.net](https://xkpasswd.net/s/) and
* [HSXKPasswd](https://www.bartbusschots.ie/s/publications/software/xkpasswd/) Perl module.

![](./docs/xkcd-936.png)

Words list for different languages are reused from [FrequencyWords](https://github.com/hermitdave/FrequencyWords).

Assets for the Web app:

* Logo from [infos-geek.com](https://infos-geek.com/en/how-to-change-the-password-on-wiko-view-2/).
* Memes from the Internet.

## Supported languages

* [English](https://github.com/xkpasswd/xkpasswd-rs/blob/main/src/assets/dict_en.txt)
* [French](https://github.com/xkpasswd/xkpasswd-rs/blob/main/src/assets/dict_fr.txt)
* [German](https://github.com/xkpasswd/xkpasswd-rs/blob/main/src/assets/dict_de.txt)
* [Portuguese](https://github.com/xkpasswd/xkpasswd-rs/blob/main/src/assets/dict_pt.txt)
* [Spanish](https://github.com/xkpasswd/xkpasswd-rs/blob/main/src/assets/dict_es.txt)

To update or add language assets, run:

```shell
$ make language-assets
```

## CLI app

### Test and build

* To test the Rust code and (simple) integration of Wasm module:

  `$ make test`

* To build:

  `$ make build`

* To run all:

  `$ make`

Then the CLI app will be available at `./target/release/xkpasswd`

_**Notes**_: all the languages will be bundled in the binary runtime.

### Usage

```
Usage: xkpasswd [OPTIONS]

Options:
  -w, --words <WORDS_COUNT>
          total number of words from dictionary

  -l, --word-min <WORD_LENGTH_MIN>
          Minimum length of a word

  -u, --word-max <WORD_LENGTH_MAX>
          Maximum length of a word

  -t, --transforms <WORD_TRANSFORMS>
          Word transformations, can be combined with multiple occurrences

          Possible values:
          - lowercase:             lowercase
          - titlecase:             Titlecase
          - uppercase:             UPPERCASE
          - inversed-titlecase:    iNVERSED tITLECASE
          - altercase-lower-first: altercase LOWER first
          - altercase-upper-first: ALTERCASE upper FIRST

  -s, --separators <SEPARATORS>
          List of characters to be used as separator

      --digits-before <PADDING_DIGITS_BEFORE>
          How many digits to be padded before the words

      --digits-after <PADDING_DIGITS_AFTER>
          How many digits to be padded after the words

  -y, --symbols <PADDING_SYMBOLS>
          List of characters to be used as padding symbols

      --symbols-before <PADDING_SYMBOLS_BEFORE>
          How many symbols to be padded before the words

      --symbols-after <PADDING_SYMBOLS_AFTER>
          How many symbols to be padded after the words

  -p, --padding <PADDING>
          Padding strategy

          Possible values:
          - fixed:
            Fixed numbers of symbols to be padded before & after words
          - adaptive:
            Pad or trim the final output to fit a length. Requires --adaptive-length.
            Notes: setting this will disable --symbols-before and --symbols-after options

  -a, --adaptive-length <ADAPTIVE_LENGTH>
          Pad or trim the final output to fit a length. Required for --padding=adaptive

  -P, --preset <PRESET>
          Possible values:
          - default:  Some sensible default values
          - apple-id: Apple ID passwords
          - ntlm:     Windows NTLM v1
          - secq:     Security questions
          - web16:    Maxium 16 characters for older websites
          - web32:    Maximum 32 characters for modern websites
          - wifi:     Fixed 63 characters for Wifi WPA2 keys
          - xkcd:     As described in the original XKCD comic

  -v, --verbose...
          Verbosity: 1 = info, 2+ = debug

  -z, --lang <LANGUAGE>
          Language of generated words

          Possible values:
          - en: English
          - fr: French
          - de: German
          - pt: Portuguese
          - es: Spanish

  -c, --config <CONFIG_FILE>
          Path to .toml config file

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Web app

The web version is deployed to https://xkpasswd.github.io.

![](./docs/xkpasswd-web.png)

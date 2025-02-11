# Change Log

User visible changes to the project will be documented here.

This project adheres to [Semantic Versioning](http://semver.org/) as described in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md).


## In progress

API:
- add formal json support
- xml/ text store methods now check for invalid string value encoding.
- load methods now check for potentially unsafe value strings.
- to_writer methods now all return crate::Result

Bugfixes:
- xml store methods now properly escape the following string characters: `\ ' < > &`.
- medium file tests now use the correct files.

Docs:
- fix typos.
- fix doc hyperlink.

Internal improvements:
- further simplify builder mechanics.
- further leverage packing traits.

## [0.2.1] 1 Feb 2025

Bugfixes: 
- 'unpack_dyn_take' now correctly works with multiple items.
- 'validate_kerning_references' error message now outputs the correct second kerning.
- 'tagged_attributes' mutate test now loops correctly.

Internal improvements:
- simplify internal builder mechanics.
- clippy fixes.


## [0.2.0] 24 Jun 2021

[LoadSettings](https://docs.rs/bmfont_rs/0.2.0/bmfont_rs/struct.LoadSettings.html) introduced along with the associated `from_xxx_ext` import methods.
This struct specifies [Font](https://docs.rs/bmfont_rs/0.2.0/bmfont_rs/struct.Font.html) import behavior, allowing us to import certain partially broken/ non-compliant BMFont files.

## [0.1.0] 10 Sep 2021

Initial release.

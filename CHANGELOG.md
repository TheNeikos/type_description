# Changelog

## v0.5.0

This is the fifth public release of the `type_description` crate.

The main addition this time are:

- Added support for BTreeMap, BTreeSet and HashSet
- Fixed support for struct enums

## v0.4.0

This is the fourth public release of the `type_description` crate.

The main addition this time are:

- Added support for the uuid crate, behind a feature-gate
- Added support for the bytesize crate, behind a feature-gate

## v0.3.0

This is the third public release of the `type_description` crate.

The main addition are the terminal renderer, which can be used to render a
`TypeDescription` to the terminal. For this the binary added v0.2.0 has been updated.

The first external type has also been added, `url::Url`. Behind a feature-gate.

## v0.2.0

This is the second public release of the `type_description` crate.

The main addition are the markdown renderer, which can be used to render a
`TypeDescription` to Markdown.

A convenience binary has also been added to transform a JSON formatted
`TypeDescription` on standard input into Markdown as standard output.

### Additions

* Markdown rendering capabilities

### New Contributors

* None this release

## v0.1.0

This is the initial public release of the `type_description` crate.

### Additions

* First release

### New Contributors

* @TheNeikos made their first contribution in https://github.com/TheNeikos/type_description/pull/1
* @matthiasbeyer made their first contribution in https://github.com/TheNeikos/type_description/pull/5
* @dependabot made their first contribution in https://github.com/TheNeikos/type_description/pull/7

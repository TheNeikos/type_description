# enterprise-rust-template

<!-- markdownlint-disable MD033 -->

[![Bors enabled](https://bors.tech/images/badge_small.svg)](https://app.bors.tech/repositories/46481)

This is an _opinionated_ template repository for rust projects.
It is "enterprise" because it contains _all the things_ you could ever want from
such a repository.

## MSRV

MSRV for this repository is currently: 1.60.0

That matters explicitely because we run some tools only for this version, e.g.:
clippy.

## Features

This section contains documentation for the "features" this repository provides.

<details>
<summary>
    Bots
</summary>

### Bors

[Bors is a GitHub bot](https://bors.tech)
that prevents merge skew / semantic merge conflicts, so when a developer
checks out the main branch, they can expect all the tests to pass
out-of-the-box.

### Dependabot

Dependabot is enabled so that dependencies are always up-to-date.

### Stalebot

Issues and PRs are automatically marked as stale by stalebot.
PRs are closed after some time, but issues are not.
</details>

<details>
<summary>
    CI builds
</summary>

Builds are done for the following distributions right now:

- [ ] alpine
- [ ] archlinux
- [ ] centos
- [ ] debian
- [ ] nixos
- [ ] RHEL
- [ ] static using musl
- [x] ubuntu
- [ ] yocto

</details>

<details>
<summary>
    CI checks
</summary>

The following checks are currently done by CI:

- [x] tests
- [x] clippy
- [ ] coverage
- [x] cargo-deny
- [x] cargo-outdated

</details>

<details>
<summary>
    CI cross compilation
</summary>

The following targets are currently enabled for cross compilation:

- [ ] ARM
- [x] RISC-V

</details>

<details>
<summary>
    Other CI features
</summary>

The following features are implemented in CI that do not fit in above
sections:

- [x] caching
- [x] commits are linted using [gitlint](https://jorisroovers.com/gitlint/)
- [x] blocking of "!fixup"/"!squash" commits
- [ ] first-time contributor message
- [ ] automatic labeling
- [ ] automatic assigning issues/PRs
- [ ] automatic reviews
  - [ ] missspell checks
  - [ ] language checks

</details>

<details>
<summary>
    GitHub Pages
</summary>

GitHub Pages are used for

- [ ] Code documentation
- [ ] website (using zola)

</details>

<details>
<summary>
    Release automation
</summary>

Nothing is implemented for release automation yet.
</details>

## License

(c) 2022 Matthias Beyer

Licensed as MIT or Apache 2.0 or MPL or WTFPL _at your opinion_.

# Changelog

## [0.2.2](https://github.com/alltuner/vacant/compare/vacant-v0.2.1...vacant-v0.2.2) (2026-05-02)


### CI/CD Changes

* add PR-CI (fmt, clippy, build, test) and clear pre-existing lints ([05aff1d](https://github.com/alltuner/vacant/commit/05aff1d7fd09d90fa78fd5b7c3cfbceb92c97d68))
* **release:** auto-bump vacant-core path-dep version constraint ([e878273](https://github.com/alltuner/vacant/commit/e878273e8c3db43d70dc0dd875f6c2937566bf97))

## [0.2.1](https://github.com/alltuner/vacant/compare/vacant-v0.2.0...vacant-v0.2.1) (2026-05-02)


### Bug Fixes

* bump vacant-core path-dep version constraint to 0.2.0 ([578f1dd](https://github.com/alltuner/vacant/commit/578f1ddbb6bbf505205981ef56092cfaec682dbe))

## [0.2.0](https://github.com/alltuner/vacant/compare/vacant-v0.1.1...vacant-v0.2.0) (2026-05-02)


### Bug Fixes

* bundle rules.toml inside vacant-cli crate so cargo publish includes it ([f4379a2](https://github.com/alltuner/vacant/commit/f4379a2f645b2f4b8a28fa16b02e7f355da3817b))

## [0.1.1](https://github.com/alltuner/vacant/compare/vacant-v0.1.0...vacant-v0.1.1) (2026-05-02)


### Bug Fixes

* **ci:** drop workspace.dependencies; declare path-dep directly in vacant-cli ([4f3a7e1](https://github.com/alltuner/vacant/commit/4f3a7e1f43800169a144b2423638f5b9909e8496))
* **ci:** give each crate a literal version so release-please can bump it ([fde9597](https://github.com/alltuner/vacant/commit/fde959719b4b12dae826e46cfe91052613627a1b))

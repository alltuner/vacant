# Changelog

## [0.3.1](https://github.com/alltuner/vacant/compare/v0.3.0...v0.3.1) (2026-05-03)


### Bug Fixes

* **deps:** bump reqwest to 0.13 ([7a038fe](https://github.com/alltuner/vacant/commit/7a038fe7fa0aa4e487d65d94cc79bb8374fbdad4))
* **deps:** bump rusqlite to 0.39 ([595cf6c](https://github.com/alltuner/vacant/commit/595cf6c3821dc8e2c433cf07f96fe31a70ae8f09))
* **deps:** update rust crate thiserror to v2 ([#16](https://github.com/alltuner/vacant/issues/16)) ([df641ce](https://github.com/alltuner/vacant/commit/df641ce1814d0efb973fa405069e993173125837))
* **deps:** update rust crate toml to v1 ([#17](https://github.com/alltuner/vacant/issues/17)) ([fdb248b](https://github.com/alltuner/vacant/commit/fdb248b7d8f3f97e36b1f54e534be40a4ef2a7e4))

## [0.3.0](https://github.com/alltuner/vacant/compare/v0.2.2...v0.3.0) (2026-05-02)


### ⚠ BREAKING CHANGES

* collapse to single crate; rewrite release pipeline

### Features

* collapse to single crate; rewrite release pipeline ([9e42709](https://github.com/alltuner/vacant/commit/9e42709f7da13951ed4b8d389d3f13df780a2f12))
* migrate to hickory 0.26 ([#7](https://github.com/alltuner/vacant/issues/7)) ([f76a633](https://github.com/alltuner/vacant/commit/f76a6338c2a1aded197532a90d665ef2b5d5f899))


### Bug Fixes

* bump vacant-core path-dep version constraint to 0.2.0 ([578f1dd](https://github.com/alltuner/vacant/commit/578f1ddbb6bbf505205981ef56092cfaec682dbe))
* bundle rules.toml inside vacant-cli crate so cargo publish includes it ([f4379a2](https://github.com/alltuner/vacant/commit/f4379a2f645b2f4b8a28fa16b02e7f355da3817b))
* **ci:** drop extra-files; release-please rust type already bumps workspace members ([77d066f](https://github.com/alltuner/vacant/commit/77d066fe22912122f2b38088467c2d92ae9f5151))
* **ci:** drop workspace.dependencies; declare path-dep directly in vacant-cli ([4f3a7e1](https://github.com/alltuner/vacant/commit/4f3a7e1f43800169a144b2423638f5b9909e8496))
* **ci:** give each crate a literal version so release-please can bump it ([fde9597](https://github.com/alltuner/vacant/commit/fde959719b4b12dae826e46cfe91052613627a1b))
* **ci:** per-crate CHANGELOG.md (release-please rejects '..' in path) ([888f94a](https://github.com/alltuner/vacant/commit/888f94a9655d043928a4b0c3b8dc287befc33da7))
* **deps:** update rust crate rand to 0.10 ([#4](https://github.com/alltuner/vacant/issues/4)) ([a1e5de5](https://github.com/alltuner/vacant/commit/a1e5de51f3c5554cb06f76f8cd3670b1ecdf9ccd))
* pack README in vacant-core tarball ([57db13b](https://github.com/alltuner/vacant/commit/57db13b878cfa238d1813a4952b06f4a364b97a7))


### Miscellaneous Chores

* add repository/homepage/keywords/categories to vacant-core ([5b468f0](https://github.com/alltuner/vacant/commit/5b468f0085e5bf73054462264f8d2fbdbb9632b6))
* release main ([#2](https://github.com/alltuner/vacant/issues/2)) ([794f4d1](https://github.com/alltuner/vacant/commit/794f4d1401550940a3af0d60556dba3894787403))
* release main ([#6](https://github.com/alltuner/vacant/issues/6)) ([ecb5ffe](https://github.com/alltuner/vacant/commit/ecb5ffeba10f2af99f8c1d1a414ff1c7e9cb264f))
* release main ([#8](https://github.com/alltuner/vacant/issues/8)) ([c24c7ba](https://github.com/alltuner/vacant/commit/c24c7ba6925602785f4838e3888797b6e656e956))
* release main ([#9](https://github.com/alltuner/vacant/issues/9)) ([b2d0299](https://github.com/alltuner/vacant/commit/b2d0299ac4481249dba9781a2a0c3e76accb2335))


### CI/CD Changes

* add cargo-dist for cross-platform binary distribution ([af86ba8](https://github.com/alltuner/vacant/commit/af86ba8a483a1116aa4920547d03de05fac9fc54))
* add PR-CI (fmt, clippy, build, test) and clear pre-existing lints ([05aff1d](https://github.com/alltuner/vacant/commit/05aff1d7fd09d90fa78fd5b7c3cfbceb92c97d68))
* add release-please workflow for automated versioning ([8e56384](https://github.com/alltuner/vacant/commit/8e56384355bfa63a83adf73ead86388951fb7806))
* align release-please config to alltuner conventions ([baedfcf](https://github.com/alltuner/vacant/commit/baedfcf00e8701adf995a9ec5b4ae94bf53b4478))
* chain dist into release-please via workflow_call (fixes 0.2.2 trigger gap) ([d65a828](https://github.com/alltuner/vacant/commit/d65a82871d33eafd98951a22cd2b613190860ee2))
* filter dist trigger to vacant-v* tags only ([56dad22](https://github.com/alltuner/vacant/commit/56dad2202b0bca91596fd9589ae42bb307e27be2))
* **release:** auto-bump vacant-core path-dep version constraint ([e878273](https://github.com/alltuner/vacant/commit/e878273e8c3db43d70dc0dd875f6c2937566bf97))
* track both crates with linked-versions; publish to crates.io on release ([bc1fe50](https://github.com/alltuner/vacant/commit/bc1fe50a940c7c82f0fcfe9dd41f5b003858485c))

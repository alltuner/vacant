# Changelog

## [0.4.0](https://github.com/alltuner/vacant/compare/vacant-v0.4.2...vacant-v0.4.0) (2026-05-04)


### Bug Fixes

* normalize URL-shaped, IDN, and dirty inputs before zone analysis ([#79](https://github.com/alltuner/vacant/issues/79)) ([12e2f5c](https://github.com/alltuner/vacant/commit/12e2f5ca5b3d0782008e1433699c5696af7e1815))
* stop caching INVALID results to prevent input-shape poisoning ([#77](https://github.com/alltuner/vacant/issues/77)) ([f3bcd39](https://github.com/alltuner/vacant/commit/f3bcd395170f94b8bde53182bf9bf4e95bcbe43c))


### Miscellaneous Chores

* convert repo to a Cargo workspace under crates/vacant ([#31](https://github.com/alltuner/vacant/issues/31)) ([ed1df6a](https://github.com/alltuner/vacant/commit/ed1df6ace80b002cff2b77381afc9404a6fc45d2))
* cut unified 0.4.0 baseline (engine + python) ([#47](https://github.com/alltuner/vacant/issues/47)) ([2db06b9](https://github.com/alltuner/vacant/commit/2db06b973b0e755e9674dafc1c74ca0833c97a8f))
* **deps:** refresh rules.toml from PSL + RDAP ([#71](https://github.com/alltuner/vacant/issues/71)) ([2c32f89](https://github.com/alltuner/vacant/commit/2c32f89407c335a81a26cbfd49d9da2376940dc4))
* **engine:** cut 0.4.0 baseline ([a6c9257](https://github.com/alltuner/vacant/commit/a6c9257c89e220262793d68ccd8c0fdc5ed30739))
* release main ([#38](https://github.com/alltuner/vacant/issues/38)) ([92cc2d8](https://github.com/alltuner/vacant/commit/92cc2d853d3bc4727f0dd85dd898e3c4945f0721))
* release main ([#49](https://github.com/alltuner/vacant/issues/49)) ([9f75f3d](https://github.com/alltuner/vacant/commit/9f75f3daf361192be2e2a4aa6ac5336f3f1a0f84))
* release main ([#72](https://github.com/alltuner/vacant/issues/72)) ([f88739d](https://github.com/alltuner/vacant/commit/f88739d424325a6f076da0c4094dc9673336a27a))
* release main ([#78](https://github.com/alltuner/vacant/issues/78)) ([69203ee](https://github.com/alltuner/vacant/commit/69203ee86bc802ab1d704d62cba85aa5daeb415c))

## [0.4.2](https://github.com/alltuner/vacant/compare/v0.4.1...v0.4.2) (2026-05-04)


### Bug Fixes

* normalize URL-shaped, IDN, and dirty inputs before zone analysis ([#79](https://github.com/alltuner/vacant/issues/79)) ([12e2f5c](https://github.com/alltuner/vacant/commit/12e2f5ca5b3d0782008e1433699c5696af7e1815))
* stop caching INVALID results to prevent input-shape poisoning ([#77](https://github.com/alltuner/vacant/issues/77)) ([f3bcd39](https://github.com/alltuner/vacant/commit/f3bcd395170f94b8bde53182bf9bf4e95bcbe43c))

## [0.4.1](https://github.com/alltuner/vacant/compare/v0.4.0...v0.4.1) (2026-05-04)


### Miscellaneous Chores

* **deps:** refresh rules.toml from PSL + RDAP ([#71](https://github.com/alltuner/vacant/issues/71)) ([2c32f89](https://github.com/alltuner/vacant/commit/2c32f89407c335a81a26cbfd49d9da2376940dc4))

## [0.4.0](https://github.com/alltuner/vacant/compare/v0.3.4...v0.4.0) (2026-05-03)


### Miscellaneous Chores

* cut unified 0.4.0 baseline (engine + python) ([#47](https://github.com/alltuner/vacant/issues/47)) ([2db06b9](https://github.com/alltuner/vacant/commit/2db06b973b0e755e9674dafc1c74ca0833c97a8f))
* **engine:** cut 0.4.0 baseline ([a6c9257](https://github.com/alltuner/vacant/commit/a6c9257c89e220262793d68ccd8c0fdc5ed30739))
* **py:** cut 0.4.0 baseline ([b1db11a](https://github.com/alltuner/vacant/commit/b1db11a90308aa071e21a64cb682889956de1e17))

## [0.3.4](https://github.com/alltuner/vacant/compare/v0.3.3...v0.3.4) (2026-05-03)


### Miscellaneous Chores

* convert repo to a Cargo workspace under crates/vacant ([#31](https://github.com/alltuner/vacant/issues/31)) ([ed1df6a](https://github.com/alltuner/vacant/commit/ed1df6ace80b002cff2b77381afc9404a6fc45d2))

## [0.3.3](https://github.com/alltuner/vacant/compare/v0.3.2...v0.3.3) (2026-05-03)


### Bug Fixes

* switch rustls crypto provider from aws-lc-rs to ring ([#30](https://github.com/alltuner/vacant/issues/30)) ([949ab9b](https://github.com/alltuner/vacant/commit/949ab9b1f3bff8f587c0f54f42e667ea829aeff4))


### Miscellaneous Chores

* **deps:** update amannn/action-semantic-pull-request action to v6 ([#28](https://github.com/alltuner/vacant/issues/28)) ([b662989](https://github.com/alltuner/vacant/commit/b662989ed6cf0162f894db65f2da9f5d46e0062b))
* drop pypi-placeholder; vacant is published from alltuner/vacant-py ([#27](https://github.com/alltuner/vacant/issues/27)) ([9e4b45d](https://github.com/alltuner/vacant/commit/9e4b45d5919210fbd0ed89a2cc5ee0d492117f48))

## [0.3.2](https://github.com/alltuner/vacant/compare/v0.3.1...v0.3.2) (2026-05-03)


### Features

* add --version flag and tighten brew formula test ([#24](https://github.com/alltuner/vacant/issues/24)) ([9e7d1f5](https://github.com/alltuner/vacant/commit/9e7d1f52dd5be7cf90c69c9c577e97c1247d25f8))


### Miscellaneous Chores

* **deps:** update astral-sh/setup-uv action to v8 ([#22](https://github.com/alltuner/vacant/issues/22)) ([a687b41](https://github.com/alltuner/vacant/commit/a687b41ae2e07937ec6d864c8a8de5f282a5e335))
* **deps:** update peter-evans/create-pull-request action to v8 ([#23](https://github.com/alltuner/vacant/issues/23)) ([fb2fa60](https://github.com/alltuner/vacant/commit/fb2fa60b3367bd8275ab2638c93ef449b3678e8d))


### CI/CD Changes

* validate PR titles as conventional commits ([#25](https://github.com/alltuner/vacant/issues/25)) ([6af0e5b](https://github.com/alltuner/vacant/commit/6af0e5b7dd8f40a3f57d7a53b74c7addb4d5a832)), closes [#20](https://github.com/alltuner/vacant/issues/20)
* weekly workflow to refresh rules.toml from PSL + RDAP ([#19](https://github.com/alltuner/vacant/issues/19)) ([d352e98](https://github.com/alltuner/vacant/commit/d352e9893cda4aec1a4d83a1ccf2d5c2feb4bf39))

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

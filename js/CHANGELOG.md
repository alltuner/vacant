# Changelog

## [0.4.11](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.10...vacant-js-v0.4.11) (2026-06-15)


### Features

* per-zone forbidden-labels policy blocklist ([#135](https://github.com/alltuner/vacant/issues/135)) ([150173c](https://github.com/alltuner/vacant/commit/150173cf64e6495fa7a18d593182cfeff355ad3b))


### Miscellaneous Chores

* **deps:** refresh rules.toml from PSL + RDAP ([#137](https://github.com/alltuner/vacant/issues/137)) ([703271b](https://github.com/alltuner/vacant/commit/703271b77ecc2d69c81b83fb0a1a8a12f8699211))

## [0.4.10](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.9...vacant-js-v0.4.10) (2026-06-15)


### Miscellaneous Chores

* **deps:** update dependency @napi-rs/cli to v3.7.2 ([#129](https://github.com/alltuner/vacant/issues/129)) ([248723c](https://github.com/alltuner/vacant/commit/248723c096be795dd47020adddbc3d8308a5b00d))
* **deps:** update dependency @types/node to v22.19.21 ([#131](https://github.com/alltuner/vacant/issues/131)) ([1d38605](https://github.com/alltuner/vacant/commit/1d38605fd7fe6b982faf340e094177130348e831))

## [0.4.9](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.8...vacant-js-v0.4.9) (2026-06-09)


### Features

* **rust,js:** add mcp stdio server subcommand ([#124](https://github.com/alltuner/vacant/issues/124)) ([a1df176](https://github.com/alltuner/vacant/commit/a1df1769f91c244cb9e5cd8fae8472e3c3e13f00))

## [0.4.8](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.7...vacant-js-v0.4.8) (2026-06-09)


### Documentation Updates

* link the hosted web UI (vacant.alltuner.com) from the READMEs ([#121](https://github.com/alltuner/vacant/issues/121)) ([250d375](https://github.com/alltuner/vacant/commit/250d3750d9b2396ef738eb7ad7e646afa81b5ad4))

## [0.4.7](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.6...vacant-js-v0.4.7) (2026-06-09)


### Miscellaneous Chores

* **deps:** refresh rules.toml from PSL + RDAP ([#112](https://github.com/alltuner/vacant/issues/112)) ([6db88e9](https://github.com/alltuner/vacant/commit/6db88e97d38aae6dc60e83a6023f47004d64d724))
* **deps:** update dependency @types/node to v22.19.20 ([#114](https://github.com/alltuner/vacant/issues/114)) ([41fcc90](https://github.com/alltuner/vacant/commit/41fcc904a37176c9e92e1107cea5890e10626f9c))


### Documentation Updates

* caveat that available isn't a registrability guarantee ([#117](https://github.com/alltuner/vacant/issues/117)) ([ca6d1e6](https://github.com/alltuner/vacant/commit/ca6d1e62cf218aedc97e6528786f2a57f2e0368f)), closes [#84](https://github.com/alltuner/vacant/issues/84)

## [0.4.6](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.5...vacant-js-v0.4.6) (2026-05-29)


### Bug Fixes

* stop reporting held domains as available; add --verify ([#104](https://github.com/alltuner/vacant/issues/104)) ([426666f](https://github.com/alltuner/vacant/commit/426666f956cd12a116037556215a77a06cb1302b))


### Miscellaneous Chores

* **deps:** harvest RDAP endpoints omitted by the IANA bootstrap ([#108](https://github.com/alltuner/vacant/issues/108)) ([06cb546](https://github.com/alltuner/vacant/commit/06cb5465d379bcfa25e69432c81a5455150e99d1))

## [0.4.5](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.4...vacant-js-v0.4.5) (2026-05-28)


### Miscellaneous Chores

* **deps:** refresh rules.toml from PSL + RDAP ([#98](https://github.com/alltuner/vacant/issues/98)) ([457d62b](https://github.com/alltuner/vacant/commit/457d62bea83d00c770905ef11514f6ce03c75ee5))
* **deps:** update dependency @napi-rs/cli to v3.7.0 ([#102](https://github.com/alltuner/vacant/issues/102)) ([64344c3](https://github.com/alltuner/vacant/commit/64344c36357e7c15ceaf3b88b1f5bbd8233d0f6c))

## [0.4.4](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.3...vacant-js-v0.4.4) (2026-05-13)


### Miscellaneous Chores

* **deps:** refresh rules.toml from PSL + RDAP ([#87](https://github.com/alltuner/vacant/issues/87)) ([aaabec7](https://github.com/alltuner/vacant/commit/aaabec718c9195a909fefe4bdd5b0bed2f533377))
* **deps:** update dependency @types/node to v22.19.19 ([#85](https://github.com/alltuner/vacant/issues/85)) ([064ad5f](https://github.com/alltuner/vacant/commit/064ad5f5428068178be29b05ba5e99b90c112ef9))

## [0.4.3](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.2...vacant-js-v0.4.3) (2026-05-04)


### Bug Fixes

* normalize URL-shaped, IDN, and dirty inputs before zone analysis ([#79](https://github.com/alltuner/vacant/issues/79)) ([12e2f5c](https://github.com/alltuner/vacant/commit/12e2f5ca5b3d0782008e1433699c5696af7e1815))
* stop caching INVALID results to prevent input-shape poisoning ([#77](https://github.com/alltuner/vacant/issues/77)) ([f3bcd39](https://github.com/alltuner/vacant/commit/f3bcd395170f94b8bde53182bf9bf4e95bcbe43c))

## [0.4.2](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.1...vacant-js-v0.4.2) (2026-05-04)


### Miscellaneous Chores

* **deps:** refresh rules.toml from PSL + RDAP ([#71](https://github.com/alltuner/vacant/issues/71)) ([2c32f89](https://github.com/alltuner/vacant/commit/2c32f89407c335a81a26cbfd49d9da2376940dc4))

## [0.4.1](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.0...vacant-js-v0.4.1) (2026-05-04)


### Bug Fixes

* **js:** drop --skip-gh-release flag (removed in napi-rs v3) ([#64](https://github.com/alltuner/vacant/issues/64)) ([6482cfe](https://github.com/alltuner/vacant/commit/6482cfedaa55654a98348dce02b670555f1e786d))
* **js:** regenerate lock with --include=optional to record stub versions ([#65](https://github.com/alltuner/vacant/issues/65)) ([2a8f2f0](https://github.com/alltuner/vacant/commit/2a8f2f0f704da1ee683dda1aca5d41d43bc899c1))


### Miscellaneous Chores

* **deps:** update dependency typescript to v6 ([#60](https://github.com/alltuner/vacant/issues/60)) ([f7eec5b](https://github.com/alltuner/vacant/commit/f7eec5b0fb1713e7fe0654bbfd7d98b25a29403b))

## [0.4.0](https://github.com/alltuner/vacant/compare/vacant-js-v0.4.0...vacant-js-v0.4.0) (2026-05-04)


### Features

* scaffold vacant-js (npm package + napi-rs cdylib) ([#53](https://github.com/alltuner/vacant/issues/53)) ([4690861](https://github.com/alltuner/vacant/commit/46908613bb228f24a7944608532647c115dcc7e7))


### Miscellaneous Chores

* **js:** cut vacant-js 0.4.0 first release ([#57](https://github.com/alltuner/vacant/issues/57)) ([8ed05ea](https://github.com/alltuner/vacant/commit/8ed05eae6ee0004278ec4abc93cac578d30fc806))

## Changelog

<!-- Initial release of @alltuner/vacant cut at the 0.4.0 unified post-monorepo baseline. -->

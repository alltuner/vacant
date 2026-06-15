# Changelog

## [0.4.11](https://github.com/alltuner/vacant/compare/vacant-py-v0.4.8...vacant-py-v0.4.11) (2026-06-15)


### Features

* per-zone forbidden-labels policy blocklist ([#135](https://github.com/alltuner/vacant/issues/135)) ([150173c](https://github.com/alltuner/vacant/commit/150173cf64e6495fa7a18d593182cfeff355ad3b))


### Miscellaneous Chores

* **deps:** refresh rules.toml from PSL + RDAP ([#137](https://github.com/alltuner/vacant/issues/137)) ([703271b](https://github.com/alltuner/vacant/commit/703271b77ecc2d69c81b83fb0a1a8a12f8699211))

## [0.4.8](https://github.com/alltuner/vacant/compare/vacant-py-v0.4.7...vacant-py-v0.4.8) (2026-06-09)


### Features

* add MCP registry metadata (server.json + mcp-name) ([#123](https://github.com/alltuner/vacant/issues/123)) ([480c3bd](https://github.com/alltuner/vacant/commit/480c3bd81cc3b5577599210e02d35280b2c49ae6))


### Documentation Updates

* link the hosted web UI (vacant.alltuner.com) from the READMEs ([#121](https://github.com/alltuner/vacant/issues/121)) ([250d375](https://github.com/alltuner/vacant/commit/250d3750d9b2396ef738eb7ad7e646afa81b5ad4))

## [0.4.7](https://github.com/alltuner/vacant/compare/vacant-py-v0.4.6...vacant-py-v0.4.7) (2026-06-09)


### Features

* **python:** add mcp stdio server subcommand ([#118](https://github.com/alltuner/vacant/issues/118)) ([b673e54](https://github.com/alltuner/vacant/commit/b673e54b770cec41775ff0cd3780cb67910e0ec2))

## [0.4.6](https://github.com/alltuner/vacant/compare/vacant-py-v0.4.5...vacant-py-v0.4.6) (2026-06-09)


### Miscellaneous Chores

* **deps:** refresh rules.toml from PSL + RDAP ([#112](https://github.com/alltuner/vacant/issues/112)) ([6db88e9](https://github.com/alltuner/vacant/commit/6db88e97d38aae6dc60e83a6023f47004d64d724))


### Documentation Updates

* caveat that available isn't a registrability guarantee ([#117](https://github.com/alltuner/vacant/issues/117)) ([ca6d1e6](https://github.com/alltuner/vacant/commit/ca6d1e62cf218aedc97e6528786f2a57f2e0368f)), closes [#84](https://github.com/alltuner/vacant/issues/84)

## [0.4.5](https://github.com/alltuner/vacant/compare/vacant-py-v0.4.4...vacant-py-v0.4.5) (2026-05-29)


### Bug Fixes

* stop reporting held domains as available; add --verify ([#104](https://github.com/alltuner/vacant/issues/104)) ([426666f](https://github.com/alltuner/vacant/commit/426666f956cd12a116037556215a77a06cb1302b))


### Miscellaneous Chores

* **deps:** harvest RDAP endpoints omitted by the IANA bootstrap ([#108](https://github.com/alltuner/vacant/issues/108)) ([06cb546](https://github.com/alltuner/vacant/commit/06cb5465d379bcfa25e69432c81a5455150e99d1))

## [0.4.4](https://github.com/alltuner/vacant/compare/vacant-py-v0.4.3...vacant-py-v0.4.4) (2026-05-28)


### Miscellaneous Chores

* **deps:** refresh rules.toml from PSL + RDAP ([#98](https://github.com/alltuner/vacant/issues/98)) ([457d62b](https://github.com/alltuner/vacant/commit/457d62bea83d00c770905ef11514f6ce03c75ee5))

## [0.4.3](https://github.com/alltuner/vacant/compare/vacant-py-v0.4.2...vacant-py-v0.4.3) (2026-05-13)


### Miscellaneous Chores

* **deps:** refresh rules.toml from PSL + RDAP ([#87](https://github.com/alltuner/vacant/issues/87)) ([aaabec7](https://github.com/alltuner/vacant/commit/aaabec718c9195a909fefe4bdd5b0bed2f533377))

## [0.4.2](https://github.com/alltuner/vacant/compare/vacant-py-v0.4.1...vacant-py-v0.4.2) (2026-05-04)


### Bug Fixes

* normalize URL-shaped, IDN, and dirty inputs before zone analysis ([#79](https://github.com/alltuner/vacant/issues/79)) ([12e2f5c](https://github.com/alltuner/vacant/commit/12e2f5ca5b3d0782008e1433699c5696af7e1815))
* stop caching INVALID results to prevent input-shape poisoning ([#77](https://github.com/alltuner/vacant/issues/77)) ([f3bcd39](https://github.com/alltuner/vacant/commit/f3bcd395170f94b8bde53182bf9bf4e95bcbe43c))


### Miscellaneous Chores

* **python:** ship _core.pyi stub and py.typed marker ([#80](https://github.com/alltuner/vacant/issues/80)) ([9f16855](https://github.com/alltuner/vacant/commit/9f16855f0d65eff56ff8145ace87e0313d467f1c))

## [0.4.1](https://github.com/alltuner/vacant/compare/vacant-py-v0.4.0...vacant-py-v0.4.1) (2026-05-04)


### Miscellaneous Chores

* **deps:** refresh rules.toml from PSL + RDAP ([#71](https://github.com/alltuner/vacant/issues/71)) ([2c32f89](https://github.com/alltuner/vacant/commit/2c32f89407c335a81a26cbfd49d9da2376940dc4))

## [0.4.0](https://github.com/alltuner/vacant/compare/vacant-py-v0.3.6...vacant-py-v0.4.0) (2026-05-03)


### Features

* **deps:** upgrade pyo3 0.22 to 0.28 ([#43](https://github.com/alltuner/vacant/issues/43)) ([50f8085](https://github.com/alltuner/vacant/commit/50f8085c1812ab4a17fbffd90706fbdbdacd7978))


### Miscellaneous Chores

* cut unified 0.4.0 baseline (engine + python) ([#47](https://github.com/alltuner/vacant/issues/47)) ([2db06b9](https://github.com/alltuner/vacant/commit/2db06b973b0e755e9674dafc1c74ca0833c97a8f))
* **py:** cut 0.4.0 baseline ([b1db11a](https://github.com/alltuner/vacant/commit/b1db11a90308aa071e21a64cb682889956de1e17))
* rename binding crate vacant-py -&gt; vacant-pyext ([#48](https://github.com/alltuner/vacant/issues/48)) ([ccae6cc](https://github.com/alltuner/vacant/commit/ccae6cc79e01022f57b70722cf9c3bae71f109b7))

## [0.3.6](https://github.com/alltuner/vacant/compare/vacant-py-v0.3.5...vacant-py-v0.3.6) (2026-05-03)


### Miscellaneous Chores

* import vacant-py into the workspace; promote rules to top-level rules/ ([#33](https://github.com/alltuner/vacant/issues/33)) ([60b4bc3](https://github.com/alltuner/vacant/commit/60b4bc3e56d29229c995ab462412d0a7dfe7791b))

## [0.3.5](https://github.com/alltuner/vacant-py/compare/v0.3.4...v0.3.5) (2026-05-03)


### Bug Fixes

* gate downstream jobs on resolved tag, not transitive success() ([0077ba0](https://github.com/alltuner/vacant-py/commit/0077ba066ab240ef595d07cefa78e2c334be9e33))

## [0.3.4](https://github.com/alltuner/vacant-py/compare/v0.3.3...v0.3.4) (2026-05-03)


### Bug Fixes

* bump manylinux to 2_28 + allow rebuilding existing tags ([#4](https://github.com/alltuner/vacant-py/issues/4)) ([218af67](https://github.com/alltuner/vacant-py/commit/218af67328d133f99e700ff0bf945eca1cfecb50))

## [0.3.3](https://github.com/alltuner/vacant-py/compare/v0.3.2...v0.3.3) (2026-05-03)


### Miscellaneous Chores

* match vaultuner renovate config shape ([#2](https://github.com/alltuner/vacant-py/issues/2)) ([a28bf3d](https://github.com/alltuner/vacant-py/commit/a28bf3d02e2b535e5c423110530d81bf1e50875e))

## [0.3.2](https://github.com/alltuner/vacant-py/compare/v0.3.1...v0.3.2) (2026-05-03)


### Features

* initial vacant-py scaffold ([3c45d69](https://github.com/alltuner/vacant-py/commit/3c45d69a01698adb70ca3f35874c4a526973d5aa))


### CI/CD Changes

* drop pypi environment from publish job ([6d560a6](https://github.com/alltuner/vacant-py/commit/6d560a680ffa2896bc3088d014e9a08fd3c47c23))

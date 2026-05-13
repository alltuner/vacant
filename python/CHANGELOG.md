# Changelog

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

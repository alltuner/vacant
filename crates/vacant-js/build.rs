// ABOUTME: Build glue for napi-rs cdylib; emits the per-platform symbols the loader needs.
// ABOUTME: Pairs with the napi v3 runtime crate; produces no artifacts when napi-build is absent.

fn main() {
    napi_build::setup();
}

# Glean, megazorded with Application Services

This is a re-packaging of the [Glean SDK](https://github.com/mozilla/glean/) for Android,
compiled in a way that works nicely with the application-services [megazord](../../docs/design/megazords.md).

The usual way for consumers to use the Glean SDK on Android is to depend on the
`org.mozilla.telemetry:glean` package, which includes both the Glean Kotlin bindings
and the compiled Rust code. However, because the Glean Rust code has been compiled
into a standalone dynamic library in this setup, it's difficult for other Rust components
to integrate with it.

By depending instead on the `org.mozilla.appservices:glean` package, consumers can
get the same Glean Kotlin bindings, but configured to load the underlying Rust code
from the application-serivces `:full-megazord` package. This lets us compile the Rust
code for Glean together with the Rust code for application-services and have them
interoperate directly at the Rust level.

To update Glean:

1. Select the release version to which to update, `$version`.
2. Update the submodule to that version (replace `$version` below with the correct version, e.g. `32.3.0`):

   ```
   cd components/external/glean
   git fetch origin
   git checkout v$version
   ```
3. Commit the changes:

    ```
    git add components/external/glean
    git add megazord/full/Cargo.toml
    git add megazord/ios/rust/Cargo.toml
    git add Cargo.lock
    ```
4. Run `./gradlew glean:assemble` to ensure that things still build correctly.
   If this returns an error, you may need to update `./build.gradle` to track
   any build changes made in Glean.
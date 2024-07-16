# tonic_example_broken_project

This project demonstrates a bug [tonic_build](https://github.com/hyperium/tonic/tree/master/tonic-build). tonic_build is unable to handle protos with services named Sync or Send due to naming collisions with [std::marker](https://doc.rust-lang.org/std/marker/index.html).

## Complication Output
```
vscode ➜ /workspaces/tonic_example_broken_project (main) $ cargo build
   Compiling tonic_example_broken_project v0.1.0 (/workspaces/tonic_example_broken_project)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
vscode ➜ /workspaces/tonic_example_broken_project (main) $ cargo build
   Compiling tonic_example_broken_project v0.1.0 (/workspaces/tonic_example_broken_project)
error[E0391]: cycle detected when computing the super predicates of `telemetry_ingester::sync_server::Sync`
  --> /workspaces/tonic_example_broken_project/target/debug/build/tonic_example_broken_project-8247014933aa3ceb/out/my_proto.rs:95:28
   |
95 |     pub trait Sync: Send + Sync + 'static {}
   |                            ^^^^
   |
   = note: ...which immediately requires computing the super predicates of `telemetry_ingester::sync_server::Sync` again
note: cycle used when checking that `telemetry_ingester::sync_server::Sync` is well-formed
  --> /workspaces/tonic_example_broken_project/target/debug/build/tonic_example_broken_project-8247014933aa3ceb/out/my_proto.rs:95:5
   |
95 |     pub trait Sync: Send + Sync + 'static {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: see https://rustc-dev-guide.rust-lang.org/overview.html#queries and https://rustc-dev-guide.rust-lang.org/query.html for more information

error[E0391]: cycle detected when computing the implied predicates of `telemetry_ingester::sync_server::Sync`
  --> /workspaces/tonic_example_broken_project/target/debug/build/tonic_example_broken_project-8247014933aa3ceb/out/my_proto.rs:95:28
   |
95 |     pub trait Sync: Send + Sync + 'static {}
   |                            ^^^^
   |
   = note: ...which immediately requires computing the implied predicates of `telemetry_ingester::sync_server::Sync` again
note: cycle used when computing normalized predicates of `telemetry_ingester::sync_server::SyncServer`
  --> /workspaces/tonic_example_broken_project/target/debug/build/tonic_example_broken_project-8247014933aa3ceb/out/my_proto.rs:97:5
   |
97 |     pub struct SyncServer<T: Sync> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: see https://rustc-dev-guide.rust-lang.org/overview.html#queries and https://rustc-dev-guide.rust-lang.org/query.html for more information

For more information about this error, try `rustc --explain E0391`.
error: could not compile `tonic_example_broken_project` (lib) due to 2 previous errors
```
# Notes


## Issues

### Profile only at root

Cargo workspace, with a members targeting WASM.
Does it make sense to have

> warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/igorlaborie/Documents/Workspaces/playground/rust-waterpouring/waterpouring-wasm/Cargo.toml
workspace: /Users/igorlaborie/Documents/Workspaces/playground/rust-waterpouring/Cargo.toml

I've move the 
```toml
[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"
```

It's remove the warning, but running `wasm-pack build`, the generated wasm is bigger.

1. the wasm-pack does not seems to get workspace profile option (and does not work at top level project)
2. How to set `opt-level` only in the package that require it

Maybe that's not a good idea to put a WASM project into a workspace.

### 


## Ideas

Syntax highligher: [syntect](https://github.com/trishume/syntect)

<https://crates.io/crates/pulldown-cmark>
<https://crates.io/crates/mdbook>

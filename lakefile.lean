import Lake
open Lake DSL

require ix from git "https://github.com/argumentcomputer/ix" @ "3c39a15dd9e92b4e46b8b53bb4e2c5438a428d6d"

package "template" where
  version := v!"0.1.0"

lean_lib «Template» where
  -- add library configuration options here

@[default_target]
lean_exe "template" where
  root := `Main

lean_exe "bench-test" where
  root := `Benchmarks.Main

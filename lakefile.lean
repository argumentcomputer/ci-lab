import Lake
open Lake DSL

require ix from git "https://github.com/argumentcomputer/ix" @ "fa2335d35cd6f71ef3d21fb67ce3d1501e9f45c5"

package "template" where
  version := v!"0.1.0"

lean_lib «Template» where
  -- add library configuration options here

@[default_target]
lean_exe "template" where
  root := `Main

lean_exe "bench-test" where
  root := `Benchmarks.Main

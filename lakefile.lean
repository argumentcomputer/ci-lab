import Lake
open Lake DSL

require ix from git "https://github.com/argumentcomputer/ix" @ "351d14d2e99542abf073d73780a01c6a4e0c9f5a"

package "template" where
  version := v!"0.1.0"

lean_lib «Template» where
  -- add library configuration options here

@[default_target]
lean_exe "template" where
  root := `Main

lean_exe "bench-test" where
  root := `Benchmarks.Main

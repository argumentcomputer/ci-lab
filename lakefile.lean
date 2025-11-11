import Lake
open Lake DSL

require ix from git "https://github.com/argumentcomputer/ix" @ "71cf57f4d1435dd0ed60bdcf3eb45677acbf839e"

package "template" where
  version := v!"0.1.0"

lean_lib «Template» where
  -- add library configuration options here

@[default_target]
lean_exe "template" where
  root := `Main

lean_exe "bench-test" where
  root := `Benchmarks.Main

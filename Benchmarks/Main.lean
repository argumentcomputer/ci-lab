import Ix.Benchmark.Bench

def sub (input: Nat): IO Nat := do
  pure $ input - 1

def subBench := bgroup "Sub" [
  benchIO "sub 1" sub 1,
  benchIO "sub 10" sub 10
] { oneShot := true }

def main : IO Unit := do
  let _result ← subBench

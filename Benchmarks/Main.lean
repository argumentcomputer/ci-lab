import Ix.Benchmark.Bench

def add' (input: Nat): IO Nat := do
  pure $ input + 1

def addBench' := bgroup "Add'" [
  benchIO "add' 1" add' 1,
  benchIO "add' 10" add' 10
]

def main : IO Unit := do
  let _result ← addBench'

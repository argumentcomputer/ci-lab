import Template
import Ix.Benchmark.Bench

def add (input: Nat): IO Nat := do
  pure $ input + 1

def addBench := bgroup "Add" [
  benchIO "add 1" add 1,
  benchIO "add 10" add 10
] { report := false }

def main : IO Unit := do
  --IO.println "hello"
  let _result ← addBench

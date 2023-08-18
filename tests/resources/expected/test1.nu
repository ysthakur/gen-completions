export extern "test1" [
  --h(-h) # Show help information
  --verbose(-v) # Verbose output
  --loud # Verbose output
]

export extern "test1 sub1" [
  --foobar # Something something
]

export extern "test1 sub2" [
  --a # Both options should be picked up even though the short one is weird
  --all # Both options should be picked up even though the short one is weird
]

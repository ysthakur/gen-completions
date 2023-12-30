def "nu-complete test-types file_path" [] {
  []
}

def "nu-complete test-types bar" [] {
  # [...[] ...[] ...['foo', 'bar', 'baz']]
  [{value: 2, description: ""}, {value: 4, description: ""}]
}

export extern "test-types" [
  --file-path(-f): string@"nu-complete test-types file_path" # File path
  --path: string@"nu-complete test-types file_path" # File path
  --bar(-b): string@"nu-complete test-types bar" # Blah blah blah
]

# The first and only subcommand
export extern "test-types subcommand1" [
  --no-args # This has no args to complete (although the flag itself should be completed)
  --unknown: string # This has an argument, but we don't know how to complete it
]

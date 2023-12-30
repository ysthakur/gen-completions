def "nu-complete test-types file_path" [] {
  []
}

def "nu-complete test-types bar" [] {
  [...[] ...((ls -l) | each { |it| {value: $it} }) ...[{value: 'foo'}, {value: 'bar'}, {value: 'baz'}]]
}

def "nu-complete test-types s" [] {
  [{value: 'asdf', description: 'Foo bar baz'}, {value: 'bleh', description: 'Lorem ipsum dolor sit amet'}, {value: 'another', description: 'Some description'}]
}

export extern "test-types" [
  --file-path(-f): string@"nu-complete test-types file_path" # File path
  --path: string@"nu-complete test-types file_path" # File path
  --bar(-b): string@"nu-complete test-types bar" # Blah blah blah
  -s: string@"nu-complete test-types s" # testing out strings with descriptions
]

# The first and only subcommand
export extern "test-types subcommand1" [
  --no-args # This has no args to complete (although the flag itself should be completed)
  --unknown: string # This has an argument, but we don't know how to complete it
]

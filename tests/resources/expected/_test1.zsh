#compdef _test1 test1

function _test1 {
    local line
    _arguments -C \
        '-h[Show help information]' \
        '--h[Show help information]' \
        '-v[Verbose output]' \
        '--verbose<foo[Verbose output]' \
        '--loud[Verbose output]' \
        ': :(sub1)' \
        '*::arg:->args'
    case $line[1] in
        sub1) _test1_sub1;;
    esac
}

function _test1_sub1 {
    _arguments \
        '--foobar[Something something]'
}

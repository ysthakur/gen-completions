# #!/usr/bin/env bash

TODO this is not how it should be generated

# function _comp_cmd_test1 {
#     COMPREPLY=()
#     case ${COMP_CWORD} in
#         1)
#             COMPREPLY=($(compgen -W '-h --help -v --verbose --loud sub1' -- $2));;
#         2)
#             case $3 in
#                 'sub1')
#                     COMPREPLY=($(compgen -W '--foobar' -- $2));;
#             esac;;
#     esac
#     return 0
# }

# complete -F _comp_cmd_test1 test1

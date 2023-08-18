#!/usr/bin/env bash

function _comp_cmd_test1 {
	COMPREPLY=()
	case $COMP_CWORD in
		1) COMPREPLY=($(compgen -W '-h --h -v --verbose --loud sub1' -- $2)) ;;
		*)
			case ${COMP_WORDS[1]} in
				sub2)
					case $COMP_CWORD in
						2) COMPREPLY=($(compgen -W '--a --all ' -- $2)) ;;
					esac
					;;
				sub1)
					case $COMP_CWORD in
						2) COMPREPLY=($(compgen -W '--foobar ' -- $2)) ;;
					esac
					;;
			esac
			;;
	esac
	return 0
}

complete -F _comp_cmd_test1 test1

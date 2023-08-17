#!/usr/bin/env bash

function _comp_cmd_test1 {
	COMPREPLY=()
	case $COMP_CWORD in
		1) COMPREPLY=($(compgen -W "-h --help -v --verbose --loud sub1" -- $2)) ;;
		*)
				case $3 in
					sub1)	COMPREPLY=($(compgen -W "--foobar" -- $2)) ;;
				esac
				;;
	esac
}

#compdef _test1 test1

function _test1 {
	local line
	_arguments -C \
		'-h[Show help information]' \
		'--h[Show help information]' \
		'-v[Verbose output]' \
		'--verbose[Verbose output]' \
		'--loud[Verbose output]' \
		': :(sub1 sub2)' \
		'*::arg:->args'
	case $line[1] in
		sub1) _test1_sub1;;
		sub2) _test1_sub2;;
	esac
}

function _test1_sub1 {
	local line
	_arguments -C \
		'--foobar[Something something]' \
		': :(nested)' \
		'*::arg:->args'
	case $line[1] in
		nested) _test1_sub1_nested;;
	esac
}

function _test1_sub1_nested {
	_arguments \
		'-co[Run a command or something]' \
		'--command[Run a command or something]' \
		'--install[Install a thing]'
}

function _test1_sub2 {
	_arguments \
		'--a[Both options should be picked up even though the short one is weird]' \
		'--all[Both options should be picked up even though the short one is weird]' \
		'-C[The short form should be picked up as -C, not -Cdirectory (example from nano)]' \
		'--backupdir[The short form should be picked up as -C, not -Cdirectory (example from nano)]'
}

#compdef _git git
function _git {
     _arguments -C \
        '-v[Prints the Git suite version that the git program came from. .sp This option ...]' \
        '--version[Prints the Git suite version that the git program came from. .sp This option ...]' \
        '--help[Prints the synopsis and a list of the most commonly used commands. If the opt...]' \
        '-h[Prints the synopsis and a list of the most commonly used commands. If the opt...]' \
        '-C[Run as if git was started in <path> instead of the current working directory....]' \
        '-c[Pass a configuration parameter to the command. The value given will override ...]' \
        '--config-env[Like -c <name>=<value>, give configuration variable <name> a value, where <en...]' \
        '--exec-path[Path to wherever your core Git programs are installed. This can also be contr...]' \
        '--html-path[Print the path, without trailing slash, where Git'"'"'s HTML documentation is ins...]' \
        '--man-path[Print the manpath (see man(1)) for the man pages for this version of Git and ...]' \
        '--info-path[Print the path where the Info files documenting this version of Git are insta...]' \
        '--paginate[Pipe all output into less (or if set, $PAGER) if standard output is a termina...]' \
        '-p[Pipe all output into less (or if set, $PAGER) if standard output is a termina...]' \
        '--no-pager[Do not pipe Git output into a pager]' \
        '-P[Do not pipe Git output into a pager]' \
        '--git-dir[Set the path to the repository (".git" directory). This can also be controlle...]' \
        '--work-tree[Set the path to the working tree. It can be an absolute path or a path relati...]' \
        '--namespace[Set the Git namespace. See gitnamespaces(7) for more details. Equivalent to s...]' \
        '--bare[Treat the repository as a bare repository. If GIT_DIR environment is not set,...]' \
        '--no-replace-objects[Do not use replacement refs to replace Git objects. See git-replace(1) for mo...]' \
        '--literal-pathspecs[Treat pathspecs literally (i.e. no globbing, no pathspec magic). This is equi...]' \
        '--glob-pathspecs[Add "glob" magic to all pathspec. This is equivalent to setting the GIT_GLOB_...]' \
        '--noglob-pathspecs[Add "literal" magic to all pathspec. This is equivalent to setting the GIT_NO...]' \
        '--icase-pathspecs[Add "icase" magic to all pathspec. This is equivalent to setting the GIT_ICAS...]' \
        '--no-optional-locks[Do not perform optional operations that require locks. This is equivalent to ...]' \
        '--list-cmds[List commands by group. This is an internal/experimental option and may chang...]' \
        '--attr-source[Read gitattributes from <tree-ish> instead of the worktree. See gitattributes...]' \
        '*::args->args'
}

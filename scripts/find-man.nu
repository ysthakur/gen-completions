let manpages = (
  man --path
    | split row ":"
    | each { |dir| glob $"($dir)/man{1,6,8}" }
    | flatten
    | each { |dir| ls $dir | get name }
    | flatten
)

# for manpage in $manpages {
#   if (open $manpage | str contains "podman") {
#     print $manpage
#   }
# }
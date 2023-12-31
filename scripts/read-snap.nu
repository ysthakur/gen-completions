# Read one of insta's snapshots and only give back the actual code, not insta's info header
def main [ file ] {
  open $file | split row "---\n" -n 3 | $in.2
}

# Records docs/media/demo.gif. Run from the repo root:
#   cargo build --release && vhs docs/demo.tape
Output docs/media/demo.gif

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 18
Set Width 1480
Set Height 600
Set Padding 24
Set Theme "Catppuccin Mocha"
Set TypingSpeed 60ms

Hide
Type 'export PATH="$PWD/target/release:$PATH" PS1="$ " && bash docs/make-demo-repo.sh /tmp/git-tagger-demo && cd /tmp/git-tagger-demo && clear'
Enter
Sleep 500ms
Show

Type "git-tagger"
Sleep 300ms
Enter
Sleep 1.5s

# Walk down to the last production release
Type "j"
Sleep 600ms
Type "j"
Sleep 1.5s

# Mark it as the base, go back up to staging, compare
Type "c"
Sleep 800ms
Type "k"
Sleep 500ms
Type "k"
Sleep 800ms
Type "c"
Sleep 3.5s

# Leave compare mode and re-sort by SemVer
Type "c"
Sleep 800ms
Type "s"
Sleep 2s
Type "q"
Sleep 500ms

# About this repo
1. Put the path to Windows executable you wish to execute in `path-to-app.txt`
2. Running `cargo run` will run that executable.
3. Protip: copy the exe from `target\debug\launch-application.exe` and `path-to-app.txt` to `C:\Windows\System32` Then you can edit the filename of `launch-application.exe` to whatever you want. This will allow you to be able to run the executable in `path-to-app.txt`'s path by simply typing the filename of `launch-application.exe` in Windows Terminal!

# Dependencies
1. [execute version 0.2.11](https://docs.rs/execute/latest/execute/)

# Why? 
I wrote this to solve a very specific problem I have : 
1. My keyboard have calculator button that when pressed , launch Windows calculator (It link to C:\Windows\System32\calc.exe by default)
2. I want to make it launch [Qalculate](https://en.wikipedia.org/wiki/Qalculate) instead.
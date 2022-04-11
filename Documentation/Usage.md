* Recording & Replaying TASes
To play a game normally, simply pass the only argument to =gm8emulator=:

#+begin_src sh
  gm8emulator path/to/game.exe
#+end_src

To start record mode, or continue a previous recording, also pass a project name with =-n=.
A folder for the project will be created in:

- Windows :: =<working-directory>/projects/=
- Linux (Near Future) :: =$XDG_DATA_HOME/opengmk/projects/= or =~/.local/share/opengmk/projects/=

#+begin_src sh
  gm8emulator path/to/game.exe -n project-name
#+end_src

While in record mode, a =save#.bin= is generated for each savestate. You can export a =save#.gmtas= file, which is for sharing, and has input data only.
If you've lost your =save#.bin=, or need to migrate OpenGMK versions, you can recreate it by simply replaying your =save#.gmtas=:

#+begin_src sh
  gm8emulator path/to/game.exe -l -f path/to/save#.gmtas -o path/to/save#.bin
#+end_src

Note that =-l= here means disabling the framelimiter so it goes by faster.

/All command-line steps will be streamlined in a future release./
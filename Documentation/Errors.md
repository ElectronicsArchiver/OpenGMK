* Load / Runtime Errors
/Loading a game gives "failed to load 'filename' - unknown format, could not identify file" or similar/

#+begin_quote
OpenGMK is made to support /GameMaker Classic/ games. It’s possible the game you are trying to load was actually made with /GameMaker: Studio/, which it does not have support for at the moment. Whether it will in the future is unclear right now.
#+end_quote

/Loading a game or while playing a game, "unimplemented kernel function" or "not yet implemented" or similar/

#+begin_quote
Your game tried to access functionality that's yet to be implemented. The full GameMaker Classic standard library is absolutely massive, and there's a good bit left to cover.
#+end_quote

/Entering record mode gives "invalid u8 while decoding bool" or "expected variant" or similar/

#+begin_quote
This means that the =save#.bin= file in your project directory is out of date with OpenGMK.
This is a byproduct of it being actively developed, and is bound to happen.

To fix it, open it in the build of OpenGMK it was created with, export a =save#.gmtas= from it,
and recreate the =save#.bin= in the new build with that as described in the recording section.
#+end_quote
# Fishies

Toy approximation of 2D flock/school behavior. Basically a variant of Boyd's
algorithm, but fun to play with! Fishes stick together in a school, but avoid
sharks when they approach.

![gameplay demo](https://github.com/SquareWave/fishies/blob/master/demo/giphy.gif?raw=true)

Key | Action
----|-------
`Q` | Close window
`P` | Pause
`R` | Rewind state
`F` | Add a fish near player
`S` | Add a shark
`Z` | Zoom in
`X` | Zoom out
`W` | Run at 2x speed
`↑` | Speed up player
`↓` | Slow down player
`←` | Turn left
`→` | Turn right


## TODO

- Get away from O(n^2) algorithm for detecting nearby fishies. Replace w/ either
  a quad tree or a grid.

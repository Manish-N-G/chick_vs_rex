## Destination file

```cli
$ chick_vs_rex --chicken 10000000 --trex 100 --simulations 1000
```

#### CHICKENS VS T-REX SIMULATOR
```text
Chickens:       10,000,000
T-Rexes:        100
Simulations:    1,000
```

#### RESULTS

```
Chicken victories:       327
T-Rex victories:         673
```

- Chicken win probability: 32.7%

#### PERFORMANCE
```
Single threaded:    8.42 s
Rayon:              1.73 s
```

- Speedup:            4.87x

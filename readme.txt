This minimal example tries to reproduce an unexpected behaviour on older rust versions.

We installed the rust toolchain 1.48 and 1.49 with:

`rustup toolchain install 1.48`

The behaviour of 1.48 is unpredictable.
The proj library is a binding to a C library. So we suspect some bad initialization.
However, the results are predictable ; it has been running of ages in our CI.
We noticed the problem when updating to rust 1.49

The result of 1.49 seems to be the right one:
we get always the same output and it is the same output as with proj’s executable `cs2cs`.

Things get even weirder: in the struct `Point`, permutting the member `lon` and `category` changes the behaviour.


With rust 1.48, we get the following output:

```
$ cargo +1.48 run

computed: 6.4735629054745925 52.73305121523669
computed: 6.473563408212647 52.73305350250336
computed: 6.473563396215215 52.73305344791958
computed: 6.473563390033239 52.733053419793926
computed: 6.473563433481753 52.733053617468244
computed: 6.473563441091686 52.73305365209057
computed: 6.473563441365634 52.73305365333693
computed: 6.473562435490775 52.733049076992316
computed: 6.473562423285016 52.73304902146088
computed: 6.47356243957953 52.73304909559455
computed: 6.473563613986603 52.73305443869738
computed: 6.473563623892076 52.73305448376357
local   : 6.473556233092426 52.733020858848604
tests   : 6.473572155087817 52.7330932977264
```

With the current stable (1.49) :

```
$ cargo +1.49 run

computed: 6.473556233092426 52.733020858848604
computed: 6.473556233092426 52.733020858848604
computed: 6.473556233092426 52.733020858848604
computed: 6.473556233092426 52.733020858848604
computed: 6.473556233092426 52.733020858848604
computed: 6.473556233092426 52.733020858848604
computed: 6.473556233092426 52.733020858848604
computed: 6.473556233092426 52.733020858848604
computed: 6.473556233092426 52.733020858848604
computed: 6.473556233092426 52.733020858848604
computed: 6.473556233092426 52.733020858848604
computed: 6.473556233092426 52.733020858848604
local   : 6.473556233092426 52.733020858848604
tests   : 6.473572155087817 52.7330932977264
```



# rs-fuenfzehn

The [Game of Fifteen](https://en.wikipedia.org/wiki/15_Puzzle) for the terminal.

```
╭────┬────┬────┬────╮
│  3 │  5 │ 11 │ 10 │
├────┼────┼────┼────┤
│ 14 │  9 │ 12 │  1 │
├────┼────┼────┼────┤
│  6 │ 13 │    │  8 │
├────┼────┼────┼────┤
│  2 │  7 │  4 │ 15 │
╰────┴────┴────┴────╯
```

## Installation

Please se the release pages for prebuild binaries for various platforms.

## How to use it

Use your arrow keys to move tiles into the blank square until all tiles are in order. You may give up on the current puzzle and quit the game by pressing "q".

When you run the game without any arguments, a standard board with 15 tiles is presented to you. For smaller or larger boards, give the desired width of the board as the first argument, e.g. `rs-fuenfzehn 8` for a board with 63 tiles.

## Why "fuenfzehn"?

Because "Fünfzehn" is "fifteen" in german.

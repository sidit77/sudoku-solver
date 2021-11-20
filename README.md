# Sudoku-solver
This solver works by keeping track of all possible values for each field. Using this technique most sudokus can be solved without ever having to backtrack.

## Example

Take this 2x2 sudoku for example:

```
┌─────┬─────┐
│   4 │   1 │
│ 1 2 │     │
├─────┼─────┤
│   1 │     │
│   3 │     │
└─────┴─────┘
```



The algorithm starts by initializing all fields with a set containing all possibilities:

````
{1, 2, 3, 4} {1, 2, 3, 4} │ {1, 2, 3, 4} {1, 2, 3, 4}
{1, 2, 3, 4} {1, 2, 3, 4} │ {1, 2, 3, 4} {1, 2, 3, 4}
──────────────────────────┼──────────────────────────
{1, 2, 3, 4} {1, 2, 3, 4} │ {1, 2, 3, 4} {1, 2, 3, 4}
{1, 2, 3, 4} {1, 2, 3, 4} │ {1, 2, 3, 4} {1, 2, 3, 4}
````

 

After that it starts adding the given numbers to the board.

Let's start with 4:

````
{1, 2, 3, 4} {         4} │ {1, 2, 3, 4} {1, 2, 3, 4}
{1, 2, 3, 4} {1, 2, 3, 4} │ {1, 2, 3, 4} {1, 2, 3, 4}
──────────────────────────┼──────────────────────────
{1, 2, 3, 4} {1, 2, 3, 4} │ {1, 2, 3, 4} {1, 2, 3, 4}
{1, 2, 3, 4} {1, 2, 3, 4} │ {1, 2, 3, 4} {1, 2, 3, 4}
````

The intuition behind this move is that we know that only 4 can occupy this field. However, this move also has consequences for the other fields in the same row, column and cell because they can not become 4 anymore:

````
{1, 2, 3   } {         4} │ {1, 2, 3   } {1, 2, 3   }
{1, 2, 3   } {1, 2, 3   } │ {1, 2, 3, 4} {1, 2, 3, 4}
──────────────────────────┼──────────────────────────
{1, 2, 3, 4} {1, 2, 3   } │ {1, 2, 3, 4} {1, 2, 3, 4}
{1, 2, 3, 4} {1, 2, 3   } │ {1, 2, 3, 4} {1, 2, 3, 4}
````

Doing this for all numbers in the top left cell leaves us with this following state:

```
{      3   } {         4} │ {1, 2, 3   } {1, 2, 3   }
{1         } {   2      } │ {      3, 4} {      3, 4}
──────────────────────────┼──────────────────────────
{   2, 3, 4} {1,    3   } │ {1, 2, 3, 4} {1, 2, 3, 4}
{   2, 3, 4} {1,    3   } │ {1, 2, 3, 4} {1, 2, 3, 4}
```

We now just learned our first new number: 3. So let's remove it from the other cells and continues adding the given numbers:

````
{      3   } {         4} │ {   2      } {1         }
{1         } {   2      } │ {      3, 4} {      3, 4}
──────────────────────────┼──────────────────────────
{   2,    4} {1         } │ {      3, 4} {   2, 3, 4}
{   2,    4} {      3   } │ {1,       4} {   2,    4}
````

At this point we have to make a guess to continue. So let's choose 2 for one of the bottom left cells:

````
{      3   } {         4} │ {   2      } {1         }
{1         } {   2      } │ {      3, 4} {      3, 4}
──────────────────────────┼──────────────────────────
{   2      } {1         } │ {      3, 4} {      3, 4}
{         4} {      3   } │ {1         } {   2      }
````

Now we have to make another choice, but however we chose the sudoku well be completed afterwards.



Now what would've happened if we made the wrong choice for 2? The answer is that we might have to backtrack and the take another path. To maximize the possibility of guessing correct this algorithm always selects the field with the least remaining choices when making a guess.
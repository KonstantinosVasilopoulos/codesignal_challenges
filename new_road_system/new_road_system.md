[Back](../README.md)

# Strings Rearrangement

https://app.codesignal.com/arcade/graphs-arcade/kingdom-roads/nCMisf4ZKpDLdHevE

## Challenge description

Once upon a time, in a kingdom far, far away, there lived a King Byteasar I. As a kind and wise ruler, he did everything in his (unlimited) power to make life for his subjects comfortable and pleasant. One cold evening a messenger arrived at the king's castle with the latest news: all kings in the Kingdoms Union had started enforcing traffic laws! In order to not lose his membership in the Union, King Byteasar decided he must do the same within his kingdom. But what would the citizens think of it?

The king decided to start introducing the changes with something more or less simple: change all the roads in the kingdom from two-directional to one-directional (one-way). He personally prepared the `roadRegister` of the new roads, and now he needs to make sure that the road system is convenient and there will be no traffic jams, i.e. each city has the same number of incoming and outgoing roads. As the Hand of the King, you're the one who he has decreed must check his calculations.

Since cartography has not yet been properly developed in the kingdom, the registers are used instead. A register is stored as a square matrix, with its size equal to the number of cities in the kingdom. If `roadRegister[i][j] = true`, then there is a road from the `ith` to the `jth` city; the road doesn't exist otherwise.

It is guaranteed that there are no looping roads, i.e. roads that lead back to the same city it originated from.

## Example

* For 

    ```
    roadRegister = [[false, true,  false, false],
                    [false, false, true,  false],
                    [true,  false, false, true ],
                    [false, false, true,  false]]
    ```

    the output should be `solution(roadRegister) = true`.

* For

    ```
    roadRegister = [[false, true,  false, false, false, false, false],
                    [true,  false, false, false, false, false, false],
                    [false, false, false, true,  false, false, false],
                    [false, false, true,  false, false, false, false],
                    [false, false, false, false, false, false, true ],
                    [false, false, false, false, true,  false, false],
                    [false, false, false, false, false, true,  false]]
    ```

    the output should be `solution(roadRegister) = true`.

## Solution

[Solved with Rust](src/main.rs)

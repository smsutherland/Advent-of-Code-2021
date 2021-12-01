# Advent of Code 2021
Advent of Code 2021 is here and so am I. I've decided to start a pattern of learning a new language for each AoC I participate in. Last year I learned Python; this year I'm learning Rust! At the time of writing (on the first day of AoC21), I have almost finished reading the [Rust Book](https://doc.rust-lang.org/stable/book/). As I go, I will try to document my thoughts on each day's solution, both in terms of the solution itself and it's implementation in Rust.

### Days
[Day 0](#day-0) / [Day 1](#day-1)

## Day 0
Before going into this, I wanted to have at least some working knowledge of how to work in Rust. I was reading through the book, but to gain some more hands-on experience, I redid some of the AoC20 problems. While doing so, I made the infrastructure used to run the solutions. [main.rs](/src/main.rs) was made to select a day to run and pass the input to the relevant function as an array of strings. While doing problem 2 in AoC20, I was faced with string decomposition. I spent considerable time trying to generalize the solution to that problem and came up with [this](/src/common.rs#L18). It's far from a perfect solution. For example, I hoped to find a way to try to parse the strings into the given type, but I couldn't find a way to work that much type wizardry. For now it simply returns a list of strings and type conversions have to be done outside the function.

## [Day 1](/src/day_1.rs)
This problem reminded me a lot of the example in the book about using iterators, so that's the route I took for my solution. I was surprised by how powerful chaining methods functions could be. The meat of the solution is only 4 "lines" long (if you bring all the chained method calls together on a single line). I like how you can go through the logic I would normally implement in a for loop, but do it all operating on the iterator as a whole.
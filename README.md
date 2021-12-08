# Advent of Code 2021
Advent of Code 2021 is here and so am I. I've decided to start a pattern of learning a new language for each AoC I participate in. Last year I learned Python; this year I'm learning Rust! At the time of writing (on the first day of AoC21), I have almost finished reading the [Rust Book](https://doc.rust-lang.org/stable/book/). As I go, I will try to document my thoughts on each day's solution, both in terms of the solution itself and it's implementation in Rust.
Note that this is written assuming you are familiar with what the problems are. You can find the 2021 problem set [here](https://adventofcode.com/2021/).

### Days
[Day 0](#day-0) / [Day 1](#day-1) / [Day 1.1](#day-11) / [Day 2](#day-2) / [Day 3](#day-3) / [Day 4](#day-4) / [Day 5](#day-5) / [Day 6](#day-6) / [Day 7](#day-7) / [Day 8](#day-8)

## Day 0
Before going into this, I wanted to have at least some working knowledge of how to work in Rust. I was reading through the book, but to gain some more hands-on experience, I redid some of the AoC20 problems. While doing so, I made the infrastructure used to run the solutions. [main.rs](/src/main.rs) was made to select a day to run and pass the input to the relevant function as an array of strings. While doing problem 2 in AoC20, I was faced with string decomposition. I spent considerable time trying to generalize the solution to that problem and came up with [this](/src/common.rs#L18). It's far from a perfect solution. For example, I hoped to find a way to try to parse the strings into the given type, but I couldn't find a way to work that much type wizardry. For now it simply returns a list of strings and type conversions have to be done outside the function.

## [Day 1](/src/day_1.rs)
This problem reminded me a lot of the example in the book about using iterators, so that's the route I took for my solution. I was surprised by how powerful chaining methods functions could be. The meat of the solution is only 4 "lines" long (if you bring all the chained method calls together on a single line). I like how you can go through the logic I would normally implement in a for loop, but do it all operating on the iterator as a whole.

## Day 1.1
Between days 1 and 2, I decided that enough was enough. I wasn't going to download the inputs manually any longer! Thus began my several hour excursion into the rabbit hole of http requests and writing files in Rust. For making http requests, I was hoping to find some simple API like the requests module in Python, but settled for the only thing I could (eventually) get to work. There was a similar process for creating and writing the file of trying desperately to understand why things aren't working until finally things come together. The whole process was a bit of a headache, but I'm glad I went through it. Even though in the grand scheme of things it's not too much code, I feel like I accomplished something.

## [Day 2](/src/day_2.rs)
This problem was perfect for the match statement. The actual logic of the problem was very quick thanks to it. Honestly, my only stumbling point today was trying to figure out how to destructure the split string in one line rather than simply doing it on multiple lines (which is what I ended up doing anyway).

## [Day 3](/src/day_3.rs)
This problem proved to be quite enigmatic for me. If you look at the day 3 file, you'll see it's the first day where I used tests. That's because there were several places where I had bugs I couldn't iron out only using the input data. However, let's go through things in chronological order.

My initial solution to part 1 involved breaking the string representation of each number into individual characters, then parsing them into numbers (either 1 or 0). Once I had that, I iterated through and summed the values in each digit into a vector with the same number of elements as there were digits. By comparing the sum to half the number of digits, I could determine whether 0s or 1s were more common. I had to iterate through the numbers and sum them up individually because of the way the two nested iterators were structured. The outer iterator went through all the numbers and the inner iterator went through the digits. If I were able to take the "transpose" of the iterators, the solution would have been much simpler. If the outer iterator went across the digits, I could map each inner iterator to it's sum and turn it into an iterator of the sums of each digit. From that point it would be the same as when I had my vector of sums. However, I am not aware of any way to transpose iterators like that. This was an instance where I thought using numpy arrays would be exceedingly effective.

Part 1 went fairly easily, but when I saw part 2, I immediately thought I had to take my "most common digit" code and extract it into its own function. Doing this proved more problematic than I had originally expected. I have since cleaned the most common bit function, but the original implementation can be found at [this commit](https://github.com/smsutherland/Advent-of-Code-2021/blob/05b67f4469ab062fddc0204d10623139f941c51c/src/day_3.rs#L51). You can still see the idea of summing the digits and comparing to length/2, but part 2 introduces the case where there is a tie. At first I was returning 2 instead of 0 or 1 with the idea that I would deal with the even cases in the usage code, but further analysis of the problem led me to change that to being able to return 0 or 1 (depending on the parity of the length). It depends on the parity of the length because if the length is odd, the .5 gets lost in the integer division, so the equal case is really a less case in disguise. The current implementation takes a different approach inspired by [Bradon Zhang](https://github.com/BradonZhang/advent-of-code-2021/blob/main/src/3.py#L15) to subtract one for each 0 rather than adding 0, then comparing the total sum to 0 instead of length/2. This achieves the same result, but in my opinion, it is a cleaner way than dealing with the parity of the length. Actually, while writing this I realized the parity of the length could be ignored entirely. Since the only case that depends on the parity was the equal case and that can only occur if there is an even count of numbers, we can assume the parity is even for that case and return a value accordingly. Getting the details of this function right was difficult, which is why 3 of the 5 tests are dedicated to making sure it works. Once I got that snafu worked out, making the actual solution for part 2 wasn't terribly hard.

I think the way things work currently could use a lot of cleaning. I may go back and do that once the advent is over, but I'm happy with it for now. I have to do a lot of type casting that I think typing my variables smarter would alleviate. There were also smaller issues that I had throughout the process. Two notable ones in my memory were dealing with a test that kept failing because I forgot to but "0b" in front of the binary number being defined in the test, and trying to work out a way to parse the string as a binary number rather than a decimal one.

## [Day 4](/src/day_4.rs)
I think I could've done this one faster than I did, but in a significantly less clean way. Instead, I tried to use the object oriented features of Rust to make the solution cleaner, rather than faster. By using an enum, I avoided having to store two boards: one with numbers and one with booleans, since I could encapsulate whether the square was marked using the enum cases and store the number inside the enum.
```Rust
enum Status {
    Marked(u64),
    Unmarked(u64),
}


struct Board {
    nums: Vec<Vec<Status>>,
}
```
Once I was able to abstract most of the important functionality into methods on the Board struct, solving both part 1 and part 2 were pretty simple.

## [Day 5](/src/day_5.rs)
I have learned from previous advents that in situations like this where you have a grid of points, rather than creating a 2d array of the entire board and storing it that way, it is better in many situations to create a hashmap who's keys are the coordinate pairs. Not only does this not require you to know the dimensions of your working space from the beginning, but it easily generalizes to higher dimensions in a memory efficient way. I know it feels weird to call a hashmap memory efficient, but since it's only storing the points of interest (as opposed to an array which necessarily stores all points), it's memory efficiency compared to an array gets better as the density of points of interest goes down. And this is on top of the aforementioned benefit of not needing to know the size when creating the hashmap. I specifically like hashmaps in Rust because of features like the `.or_insert` method. The number of times I've written code like `if x not in map: map[x] = 0` pains me. `.or_insert` creates the same behavior, but in a much cleaner way. Then, doing the final count was as easy as creating a filter on the iterator across the hashmap.

## [Day 6](/src/day_6.rs)
As I assume many people did on today's puzzle, I did part 1 in way which is very inefficient, but switch over to a better solution for part 2. I've kept my original solution to part 1 commented out in the solution file. The original solution was done by mapping each fish to the number beneath it, then keeping a count of how many new fish get born, then appending that number of fish. The second solution only has a vector of 9 values, each storing the number of fish with that internal counter (0 through 8, inclusive). Using a drain iterator, I'm able to effectively roll the values down by one, then add on the new fish. It's a similar idea to using `np.roll` in Python. However, this problem's solution was larger than an unsigned 32 bit integer can hold, so I had to switch from using u32s to using u64s. Because of the way the AoC infrastructure I made works (see [Day 0](#day-0)) and Rust's static typing, I had to retroactively change all previous solutions from using u32s to u64s. Not a difficult task by any measure, but an unfortunate consequence.

## [Day 7](/src/day_7.rs)
I did pretty good on today's problem: top 700 for part 2. This was because of how similar parts 1 and 2 were, so solving part 2 mainly entailed copy-pasting my solution to part 1 and changing `(i - x).abs()` to `(i - x).abs() * ((i - x).abs() + 1) / 2`. When I initially did the problem, I looped through the range 0 through max and kept track of the minimum fuel level, but I later went back and converted the loop to an iterator operation on the given range. Because of the simplicity of the problem, the super-iterator solution can be done in a single expression, so the answers are calculated in the return value; the first line of the function is a solution which truly does the whole problem in a single line. This is just a flattening of the prettified solution below it, but it does mean that everywhere that uses the nums variable in the pretty solution has to go and parse the input again. In total, this one-line solution is ~500 characters long. Overall, an easy problem. I am growing ever fonder of iterators in Rust with every problem that can be solved by leveraging them.

## [Day 8](/src/day_8.rs)
Part 1 was pretty easy, so I will spent my time here discussing part 2. One key thing to note was that the solving path is the same regardless of what the input is. The process goes something like this:
1. We know which of the input is 1, 7, 4, and 8 because they have a unique number of segments activated.
2. Top = 7 - 1 (note that the arithmetic is done on the sets of segments making up each digit, not the literal numbers).
3. 4 + 7 = 9 - Bot
   - For this, there are two unknowns, Bot and 9. However, 9 is the only number which has one extra segment beyond 4 + 7. Therefore, by finding which number for whom |x - 4 - 7| = 1, x must be 9 and the one element must be Bot. This operation is performed in the `find_one_more` function.
4. BotL = 8 - 9
5. 7 + Bot + BotL = 0 - TopL
6. 4 - 1 - TopL = Mid
7. Top + Mid + Bot + BotL = 2 - TopR
8. BotR = 8 - AllOtherSides

Once we know which letter corresponds with which side, we invert the key to get the reverse key and decrypt the numbers given.
There are certainly other solving paths that work just as well, but I don't think it can get any shorter, since each step beyond the first one involves finding one of the seven sides and I don't think it's possible to find multiple sides in a single step.
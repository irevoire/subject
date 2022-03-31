# Meilisearch internship

Hello folks, if you're reading this, it's because you're currently interviewing at Meilisearch.

The aim of this technical test is to follow your train of thought when you need to work on already existing code.
You should not spend more than one hour on the test. What will be watched is not how polished your solution is but what you tried to improve.


## The problem

We're going to model the population growth of lanternfishes.
Here are the rules;
- A lanternfish never dies
- A lanternfish reproduce itself every **seven** days
- A **new** lanternfish need slightly longer before it's capable of producing more lanternfish: two more days for its first cycle.

--------

So, suppose you have a lanternfish with an internal timer value of 3:
- After one day, its internal timer would become 2.
- After another day, its internal timer would become 1.
- After another day, its internal timer would become 0.
- After another day, its internal timer would reset to 6, and it would create a new lanternfish with an internal timer of 8.
- After another day, the first lanternfish would have an internal timer of 5, and the second lanternfish would have an internal timer of 7.

------

In the code we start with an initial list of lanternfishes: `[LanternFish { timer: 3 }, LanternFish { timer: 4 }, LanternFish { timer: 3 }, LanternFish { timer: 1 }, LanternFish { timer: 2 }]`.
Here is an example of the first four iterations and how the list should evolve.

- After 0 days there is 5 lanternfishes: `[LanternFish { timer: 3 }, LanternFish { timer: 4 }, LanternFish { timer: 3 }, LanternFish { timer: 1 }, LanternFish { timer: 2 }]`
- After 1 days there is 5 lanternfishes: `[LanternFish { timer: 2 }, LanternFish { timer: 3 }, LanternFish { timer: 2 }, LanternFish { timer: 0 }, LanternFish { timer: 1 }]`
- After 2 days there is 6 lanternfishes: `[LanternFish { timer: 1 }, LanternFish { timer: 2 }, LanternFish { timer: 1 }, LanternFish { timer: 6 }, LanternFish { timer: 0 }, LanternFish { timer: 8 }]`
- After 3 days there is 7 lanternfishes: `[LanternFish { timer: 0 }, LanternFish { timer: 1 }, LanternFish { timer: 0 }, LanternFish { timer: 5 }, LanternFish { timer: 6 }, LanternFish { timer: 7 }, LanternFish { timer: 8 }]`
- After 4 days there is 9 lanternfishes: `[LanternFish { timer: 6 }, LanternFish { timer: 0 }, LanternFish { timer: 6 }, LanternFish { timer: 4 }, LanternFish { timer: 5 }, LanternFish { timer: 6 }, LanternFish { timer: 7 }, LanternFish { timer: 8 }, LanternFish { timer: 8 }]`

## What you need to solve

Now what you need to do is take a look at our [solution](src/main.rs) to this problem.
Everything looks correct, but when I try to compute the number of fish after 1000 days, my program stops.

Can you explain why? Can you fix that?


To execute the code run `cargo run`.

Feel free to change EVERYTHING in the code. You can create or get rids of methods, structs, loops, iterators, etc.
The only things you need to keep are the two final prints.

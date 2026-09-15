### About:
This project is used to simulate the fight between chickens and Trexes.

Although we could expland this to many different animals, for the sake for simplicity, we stick to these two.

The goal being how to can use testing, logging and bench tools to see how our program works depending on the differnt values sent.

This is a CLI application, and we can send user input arguements to determine the values.

This project was mainly created as a presentation, and has presentation files to show how we go about the creation of a rust project and how we use the tools acessable in the rust ecosystem.

The presentation file is also attached to this project.

### Use:
Just clone this project into your system.

You can access the presentation made for this talk.

We can then run the project with these variables.
```text
cargo run -- --chicken 10000000 --trex 3680

// or

chick_vs_rex --chicken 10000000 --trex 100

// or 

chick_vs_rex --chicken 10000000 --trex 100 --print true
```
This rougly tell us that we want to simulate a fight between 10 million chickens and 3680 trexes.

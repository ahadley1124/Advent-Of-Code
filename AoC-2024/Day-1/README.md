# Day 0 Template

This is a template day that will be used to copy the structure for the other days. It will contain the README.md file and the folders for the Rust and V code. The README.md file will contain the instructions for the day and links to the input and output files.

## Instructions

Part 1:
Part one was sorting the list, finding the difference in number and adding the differences together. For Rust, I created two vectors, each representing a column. I then sorted the vectors, used a for loop to iterate through, and ran a quick check when subracting to only have the positive number. I then added the positive number to a sum variable.

Part 2:
Part two was finding a similarity score beetween the two columns. This was a little bit more annoying because rust doesn't like Vec<i32, i32> so i had to use a Vec<(i32, i32)>. I then found created another tuple and found the number of time a number was the same in both columns and added that to a different tuple. then i iterated through the first column and the similarity score tuple and added the score to a result variable for the final answer.

## Input

The input for the day can be found [here](input.txt).

## Output

The output for the day can be found [here](output.txt).

## How to run the code

### Rust

To run the Rust code, you will need to have Rust installed on your machine. You can install Rust by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can run the code by navigating to the day you want to run and running the following command:

```bash
cargo run --release
```

### V

To run the V code, you will need to have V installed on your machine. You can install V by following the instructions on the [V website](https://vlang.io/).
It is recommended to install V from the source and not the pre-built binaries as the pre-built binaries are not always up to date.

Once you have V installed, you can run the code by navigating to the day you want to run, enter the V directory and run the following command:

```bash
v run .
```

## License

This repository is licensed under the MIT license. See the [LICENSE](../../LICENSE) file for details.

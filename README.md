# Lines Count

## Usage

example:

```cargo run --release ../linux .c .h```

The first argument specifies the directory of the project.
Other arguments specify the file suffix we are concerned about.
The above means counting the number of lines of code in files whose name ends with `.c` and `.h` (i.e., all the C files and C header files) in directory (and all of its subdirectory) `../linux`.

## Note

After parallelization, it becomes IO-bound, so setting the number of *jobs* (*NUM_JOBS*) to be more than the number of CPU cores helps.
The performance is very robust to *NUM_JOBS*, as long as it is not too small.
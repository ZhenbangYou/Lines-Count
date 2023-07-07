# Lines Count

## Usage

example:

```cargo run --release ../linux .c .h```

The first argument specifies the directory of the project.
Other arguments specify the file suffix we are concerned about.
The above means counting the number of lines of code in files whose name ends with `.c` and `.h` (i.e., all the C files and C header files) in directory (and all of its subdirectory) `../linux`.

## Note

After parallelization, it becomes IO-bound.
Now we use *rayon* to automatically do parallelism, so there is no need to worry about hyper-parameters.

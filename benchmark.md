Just out of curiosity I updated the Go implementation and added Ruby and JS versions.

All of the implementations are inefficient as they reallocate the world object for each step.

At default settings:

Go: 0.12528325
JS: 0.479
Ruby: 11.157064

Go is 3.8x faster than JS and 89.3x faster than Ruby.
JS is 23.3x faster than Ruby.

This is obviously a toy benchmark. The underlying algorithm could be optimized and there may be language specific improvements as well. But it is still interesting to see the naive performance of a compute and memory bound application.

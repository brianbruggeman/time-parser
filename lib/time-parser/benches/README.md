The motivation for this small project was two-fold:

- I wanted to port an approach I had used previously in Python to Rust as an exercise
- I wanted to see how much faster Pest was over Regex

Initial Timings on Macbook Pro (M1, 2020):

| Approach | Parse     | Timing | Improvement   |
|----------|-----------|--------|---------------|
| Pest     | hms       | 0.5µs  | 188x          |
| Pest     | shorthand | 1.8µs  |  89x          |
| Pest     | both      | 1.9µs  | 147x          |
|----------|-----------|--------|---------------|
| Regex    | hms       | 94µs   | baseline      |
| Regex    | shorthand | 161µs  | baseline      |
| Regex    | both      | 280µs  | baseline      |

The results were very surprising for me.  Pest was significantly faster than Regex.  While I do not think that the timing generally matters for the usecase I originally had in mind, I do think there are other use-cases where the performance could matter.

Digging in, I then tried to bench the python implementation.  To my surprise, the python implementation was actually faster than the Rust Regex, which made no sense initially.

But it turns out that building a Regex in rust is actually very time consuming.  So the cost needs to be amortized over the lifetime of the Regex.  To do that, I used once cell.  When I repeated the benchmark, regex ended up faster than Pest.

Updated Timings on Macbook Pro (M1, 2020):

| Approach      | Parse     | Timing | Improvement   |
|---------------|-----------|--------|---------------|
| Pest          | hms       | 0.5µs  | 188x          |
| Pest          | shorthand | 1.8µs  |  89x          |
| Pest          | both      | 1.9µs  | 147x          |
|---------------|-----------|--------|---------------|
| Regex(old)    | hms       | 94µs   | baseline      |
| Regex(old)    | shorthand | 161µs  | baseline      |
| Regex(old)    | both      | 280µs  | baseline      |
|---------------|-----------|--------|---------------|
| Python(regex) | hms       | 1.13µs | 83x           |
| Python(regex) | shorthand | 3.91µs | 41x           |
| Python(regex) | both      | 5.19µs | 54x           |
|---------------|-----------|--------|---------------|
| Regex(amo)    | hms       |   82ns | 1146x         |
| Regex(amo)    | shorthand |  787ns | 204x          |
| Regex(amo)    | both      | 1090ns | 257x          |

That said, Pest is actually doing more work at the moment than the Regex implementation.  So I decided to also benchmark the Pest implementation that is exposed within Python.


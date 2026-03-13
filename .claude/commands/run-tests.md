Build the Rust scheduler and run all 9 test cases, reporting pass/fail for each.

Steps:
1. Run `export PATH="/c/Users/gokug/.cargo/bin:/c/Users/gokug/scoop/apps/mingw/current/bin:$PATH" && cargo build 2>&1` from the project root. If it fails, show the error and stop.
2. For each of these 9 test cases in order:
   - c2-fcfs, c2-sjf, c2-rr
   - c5-fcfs, c5-sjf, c5-rr
   - c10-fcfs, c10-sjf, c10-rr

   Run: `./target/debug/scheduler pa1-testfiles/<name>.in > /tmp/<name>.out 2>&1`
   Then: `diff /tmp/<name>.out pa1-testfiles/<name>.out`

3. Show a summary table at the end:
   ```
   Test       | Result
   -----------|-------
   c2-fcfs    | PASS
   c2-sjf     | FAIL
   ...
   ```
4. For each FAIL, show the full diff output (lines prefixed with < are actual, > are expected).

Build the Rust scheduler and run a single test case. Usage: /run-test c2-fcfs

The test case name is: $ARGUMENTS

Steps:
1. Run `export PATH="/c/Users/gokug/.cargo/bin:/c/Users/gokug/scoop/apps/mingw/current/bin:$PATH" && cargo build 2>&1`. If it fails, show the compiler error and stop.
2. Run: `./target/debug/scheduler pa1-testfiles/$ARGUMENTS.in > /tmp/$ARGUMENTS.out 2>&1`
3. Run: `diff /tmp/$ARGUMENTS.out pa1-testfiles/$ARGUMENTS.out`
4. If diff output is empty: print "PASS ✓"
   Otherwise: print "FAIL" and show the full diff.
   Lines starting with `<` are your actual output; lines starting with `>` are the expected output.

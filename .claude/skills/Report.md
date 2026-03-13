# SKILL: Generate PA1 ChatGPT Scheduler Final Report

## Overview

Your task is to write a final group report for PA1 - The ChatGPT Scheduler, a UCF Computer Engineering assignment where teams used ChatGPT to generate Rust code implementing three CPU scheduling algorithms (FCFS, preemptive SJF, and Round Robin).

The report will be submitted as a Word (.docx) or PDF file titled `Group-N Final Report.docx` (replace N with the actual group number). It must be written by a human student — not sound like AI output. Follow the humanizer rules at the end of this skill.

---

## Step 1: Gather Context from the Repository

Before writing anything, read the project files:

1. **Read the Rust source file** (`scheduler-gpt.rs` or similar) — understand what the code does, how it's structured, what the algorithms look like, whether comments exist, what variable names are used.
2. **Read any prompt history files** — look for `.txt`, `.md`, or any files documenting ChatGPT conversations, prompt iterations, or notes about the process.
3. **Check the git log** (`git log --oneline`) to understand what changed over time.
4. **Run the code** against test inputs if possible to observe actual output behavior:
   ```
   cargo build --release
   ./scheduler-gpt inputFile.in
   ```
5. **Note any human-written sections** that were manually added or modified (these should be commented in the source).

Use everything you find to write a grounded, specific report. Avoid vague or generic statements — cite actual things you observed in the code.

---

## Step 2: Report Structure

The report must be titled **"Group-N Final Report"** and list all team members at the top, followed by links to each member's ChatGPT conversation.

Then address each of the six evaluation criteria below as its own section. Each section is worth 10 points.

---

### Section 1: Correctness

**Rubric question:** Does the code perform the intended task correctly? Are there any bugs or errors that need to be fixed?

**What to write:**
- State clearly whether the final code produced correct output on the test files for all three algorithms (FCFS, SJF preemptive, RR).
- Be specific: "FCFS produced correct turnaround times across all test cases." or "The initial RR implementation had an off-by-one error in quantum counting that we caught when testing the 5-process input."
- Describe how you verified correctness — did you compare against expected `.out` files? Did you manually trace through outputs?
- If ChatGPT generated code with bugs, say exactly what the bug was and how it was resolved (via re-prompting or manual fix).
- Mention any edge cases: what happens when a process arrives after the runfor window? What about ties in arrival time? Did the code handle missing parameters with proper error messages?

---

### Section 2: Efficiency

**Rubric question:** Is the code efficient and does it avoid unnecessary computations or data structures? Can it be optimized?

**What to write:**
- Look at the actual scheduling logic in the Rust file. Does it sort processes on every tick? Does it use a priority queue or linear scan?
- Comment on whether the data structures chosen (Vec, BinaryHeap, etc.) are appropriate for the problem size.
- Note if the code does any redundant work — e.g., iterating through all processes on every time tick when only a subset is eligible.
- This is a binary rubric (10 pts or 0 pts), so address it directly. If the code is reasonably efficient, say so with justification. If not, describe what could be improved.

---

### Section 3: Readability

**Rubric question:** Is the code well-organized, well-documented, and easy to understand? Does it follow best practices like meaningful variable names and consistent style?

**What to write:**
- Describe the naming conventions used (camelCase, snake_case — Rust convention is snake_case).
- Note whether comments were present — were they in the generated output, or did the team add them?
- Describe the overall file structure: is there one `main()` that does everything, or are the algorithms broken into separate functions?
- Be specific: "The `run_fcfs` function is 40 lines and handles arrival checking, selection, and output within a single loop" is more useful than "the code was readable."
- Mention consistency across the three algorithm implementations — do they follow the same patterns?

---

### Section 4: Completeness

**Rubric question:** Does the code handle edge cases and error conditions? Is it flexible enough for different inputs? What happens with malformed inputs? Does it handle race conditions (two processes schedulable at the same time)?

**What to write:**
- Does the code check for the missing `quantum` parameter when `use rr` is specified and print the correct error message?
- Does it check for missing `processcount`, `runfor`, or `use` directives?
- What happens if `use` has an invalid value?
- What happens if the file doesn't exist — does it print the usage message?
- Tie-breaking: when two processes arrive at the same time or have equal burst times, what does the code do? Is this behavior consistent and documented?
- What happens to processes that don't finish before `runfor` — does the code print "X did not finish"?

---

### Section 5: Learning Assistance

**Rubric question:** Did you need AI assistance or additional support in learning Rust? Was the AI-generated code clear enough to understand without learning Rust? Do you think it's possible to use AI to code in languages you don't know? What limitations do you foresee?

**What to write:**
- Be honest about your Rust background — were you learning it for this class, had you seen it before, or was this your first exposure?
- Describe whether you felt you needed to understand the Rust code to work with it, or whether you could treat it as a black box.
- Give a concrete opinion: could you use ChatGPT to build something in a language you don't know at all? Where does that break down?
- Think about ownership/borrow checker issues in Rust specifically — did ChatGPT generate code that compiled cleanly, or did you run into compiler errors that were hard to debug without knowing the language?
- This section invites personal reflection. Write in first person. Have an actual opinion.

---

### Section 6: Overall Quality and Final Evaluation

**Rubric question:** How would you rate the overall quality of the code? Would you recommend changes? How would you rate your experience coding with AI? Was it easier or harder than expected? What did you learn? What would you do differently?

**What to write:**
- Give an honest overall rating — not a vague "it went well." Say specifically where ChatGPT did well (structure, boilerplate, algorithm logic) and where it fell short (edge cases, Rust idioms, error handling).
- Describe the prompt iteration process: how many prompts did it take to get working code? What caused you to re-prompt? What kinds of re-prompts worked?
- Was it easier or harder than writing the code yourself would have been? Be honest — if it saved time on some parts but created confusion elsewhere, say both.
- What would you do differently? (e.g., start with more detailed prompts, test earlier, not accept the first output)
- End with a concrete takeaway about AI-assisted coding, not a feel-good platitude.

---

## Step 3: Formatting

- Use a Word `.docx` file (use the `python-docx` library or equivalent).
- Title at top: **Group-N Final Report** (replace N).
- List all team members below the title.
- List all ChatGPT conversation links (one per member, if available — otherwise leave a placeholder `[LINK]`).
- Use clear section headers matching the six criteria above.
- Length: aim for 300–500 words per section. Enough to be thorough, not padded.
- No bullet-point-heavy sections — write in paragraphs. Lists are fine sparingly.

---

## Step 4: Humanize the Report

After drafting the report, apply the humanizer process below. This is critical — the report will be read by a professor and must not read like AI output.

### Humanizer Rules (from humanizer SKILL)

**Remove these patterns entirely:**

- Significance inflation: "testament to," "pivotal moment," "underscores," "highlights," "reflects broader trends," "evolving landscape"
- Promotional language: "groundbreaking," "vibrant," "robust," "seamlessly," "powerful," "stunning," "remarkable"
- Vague attributions: "experts argue," "observers note," "industry reports suggest"
- Superficial -ing phrases tacked on for fake depth: "highlighting the importance of X," "contributing to better outcomes," "showcasing how AI can..."
- Rule of three / synonym cycling: "efficient, scalable, and maintainable"
- Copula avoidance: "serves as," "functions as," "stands as" → use "is/are"
- Em dashes used for dramatic pauses
- Generic conclusions: "the future looks bright," "exciting times ahead," "this was a valuable learning experience"
- Filler phrases: "it is important to note that," "at its core," "in order to," "due to the fact that"
- Excessive hedging: "could potentially possibly be argued that"
- Chatbot artifacts: "great question," "I hope this helps," "let me know if you need anything"

**Add these qualities:**

- **Varied sentence rhythm.** Short sentences. Then longer ones that breathe and take their time. Mix them.
- **Specific observations over vague claims.** "The SJF implementation uses a linear scan on every tick" is better than "the code could be more efficient."
- **Actual opinions.** "Honestly, the first output from ChatGPT was surprisingly close — we only needed two more prompts to fix the output formatting" is better than "the AI performed adequately."
- **First person where appropriate.** This is a team reflection. Use "we" and "I."
- **Acknowledge mixed feelings.** "It saved time on the boilerplate but made debugging harder because we weren't sure which parts of the Rust code we could actually touch."

**Final anti-AI pass:**
After writing the full draft, ask yourself: "What makes this obviously AI-generated?" Fix whatever you find. Common remaining tells:
- Every paragraph is the same length
- All sentences are about the same complexity
- No uncertainty, no hedging, no "we're still not totally sure why this works"
- Reads like a product announcement instead of a student reflection

---

## Step 5: Output

Save the final report to the outputs directory as:
```
Group-N Final Report.docx
```

If you cannot determine the group number from the repository, use `Group-X Final Report.docx` and note the placeholder in your response.

The report should be grounded in what you actually observed in the code and process — not generic statements about AI or scheduling algorithms. If something was confusing, say it was confusing. If a prompt failed and you had to try again, describe what failed. Specificity is what earns full marks on this rubric.
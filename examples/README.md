# Example scripts

Sample `.txt` files for testing OpenPrompter's **Import** button and prompter.

| File | Use |
|------|-----|
| `scripts/welcome_demo.txt` | General feature walkthrough (pauses, mirror, speed). |
| `scripts/conference_keynote.txt` | Longer talk with `[PAUSE]` markers. |
| `scripts/short_test.txt` | Tiny 3-line script for quick smoke tests. |

## How to test

1. Open OpenPrompter.
2. Click **Import** in the script library.
3. Pick one of the files above.
4. The title is taken from the filename (extension stripped).
5. Open the script and start the prompter.

These same files are used by the automated full-flow tests in
`src-tauri/tests/full_flow_tests.rs` (import round-trip).

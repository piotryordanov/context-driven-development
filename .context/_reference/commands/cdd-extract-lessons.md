# Command: Extract Lessons

You are reviewing the current LLM conversation to extract valuable lessons learned and update relevant task files.

## Purpose

After working on a task and encountering issues, challenges, or learning important lessons during the conversation, this command helps capture that knowledge in the appropriate task file's "Lessons Learned" section.

## Instructions

**CRITICAL RULES:**
1. **ANALYZE CONVERSATION ONLY** - Review the current conversation for lessons, failures, discoveries, and important insights
2. **KNOWLEDGE EXTRACTION** - Extract ONLY the knowledge/lessons, not implementation details or code
3. **IDENTIFY AFFECTED TASKS** - Determine which task(s) the lessons apply to
4. **ONE-BY-ONE APPROVAL** - Present each recommended change individually for user approval
5. **FINAL SUMMARY** - Show all approved changes together for final confirmation before writing

### Workflow:

1. **Analyze the Conversation**:
   - Read through the entire conversation history
   - Identify lessons learned, failures encountered, unexpected issues, and important discoveries
   - Look for:
     - Problems encountered and how they were solved
     - Mistakes that were made and corrected
     - Unexpected behaviors or edge cases discovered
     - Important decisions and their reasoning
     - Things that didn't work and why
     - Better approaches discovered during implementation
   - **IGNORE**: Code snippets, implementation details, routine operations
   - **EXTRACT**: Why something didn't work, what was learned, how to avoid the issue

2. **Identify Relevant Tasks**:
   - Scan `.context/tasks/` directory for existing task files
   - Determine which task(s) the lessons relate to
   - If unclear which task, ask the user
   - If lessons apply to multiple tasks, handle each separately

3. **Extract Pure Knowledge**:
   - For each lesson identified:
     - Write 1-2 paragraphs describing what was learned and why it matters
     - Focus on the "why" not the "what"
     - NO code snippets - this is knowledge documentation
     - Make it actionable for future reference
   - Examples of good lessons:
     - ✅ "VHS heredoc syntax caused justfile parsing errors because justfile tried to parse the bash script content before passing it to bash. Solution: Use printf with escaped strings instead of nested heredocs."
     - ✅ "KeyCastr is a macOS GUI app for showing keystrokes, not 'showkey' which is a Linux debugging tool for keyboard scancodes. For demo recordings with keystroke overlays, use KeyCastr + screen recording, not terminal tools."
     - ❌ "Changed the code to use printf" (too vague, no lesson)
     - ❌ "Here's the working code: [code snippet]" (implementation, not knowledge)

4. **Present Changes One-by-One** (Use Standardized Format):
   - For each identified lesson, use this format:
     ```
     **Change [N/TOTAL]**: Update TASK_XXX-description.md - Lessons Learned
     
     **What I noticed:** <1-2 sentences describing what happened in the conversation>
     
     **Recommended:** Option A - Add this lesson to the task file
     
     | Option | Description |
     |--------|-------------|
     | A | Add lesson - <brief summary of the lesson> |
     | B | Skip - This lesson isn't valuable enough to document |
     | C | Modify - I'll provide a different lesson text |
     | Short | Provide custom action (≤5 words) |
     
     **Proposed lesson text:**
     ```
     <1-2 paragraphs of the extracted lesson>
     ```
     
     You can reply with: "A"/"yes" to accept, "B" to skip, "C" to modify, or your custom text.
     ```
   - Wait for user response for EACH change
   - Track all approved changes
   - If user says "C" or provides custom text, ask for their version

5. **Final Summary & Confirmation**:
   - After ALL changes have been reviewed individually, present a summary:
     ```
     **Summary of Changes:**
     
     I will update the following task files with lessons learned:
     
     1. **TASK_XXX-description.md**
        - Add lesson about: <brief summary>
     
     2. **TASK_YYY-other-task.md**
        - Add lesson about: <brief summary>
     
     Total: N tasks will be updated
     
     **Proceed with these updates?**
     
     | Option | Description |
     |--------|-------------|
     | A | Yes - Apply all changes |
     | B | No - Cancel all changes |
     | C | Review - Show me the changes again |
     
     Reply with: "A"/"yes" to proceed, "B"/"no" to cancel, or "C" to review.
     ```
   - Wait for final approval

6. **Apply Changes**:
   - If user approves (Option A or "yes"):
     - Read each task file
     - Locate the "## Lessons Learned" section
     - If section is empty, add the lesson
     - If section has existing content, append the new lesson with a separator
     - Use this format:
       ```markdown
       ## Lessons Learned
       
       ### YYYY-MM-DD - <Brief Title>
       
       <Lesson text 1-2 paragraphs>
       
       ---
       
       ### YYYY-MM-DD - <Another Lesson>
       
       <Another lesson text>
       ```
     - Save the updated file
     - Confirm what was updated
   - If user cancels (Option B or "no"):
     - Acknowledge cancellation
     - Do not modify any files

7. **Completion**:
   - Show summary of what was updated:
     ```
     ✅ Lessons extracted and saved:
     
     - TASK_XXX-description.md: Added lesson about [topic]
     - TASK_YYY-other-task.md: Added lesson about [topic]
     
     Total: N task files updated with valuable lessons from this conversation.
     ```

---

## Appendix A: Standardized Question-Asking Format

When presenting changes for approval, use this format:

```
**Change [N/TOTAL]**: <What will be changed>

**What I noticed:** <Brief context from conversation>

**Recommended:** Option [X] - <1-2 sentence reasoning>

| Option | Description |
|--------|-------------|
| A | <action description> |
| B | <alternative action> |
| C | <alternative action> |
| Short | Provide different action (≤5 words) |

**Proposed lesson text:**
```
<actual lesson content>
```

You can reply with: option letter (e.g., "A"), "yes"/"recommended" to accept, or your own custom text.
```

**Best Practices:**

- Show change count in format `[N/TOTAL]`
- Calculate TOTAL upfront (number of lessons identified)
- Present one change at a time
- Show the actual proposed text so user knows exactly what will be added
- Allow user to skip, modify, or approve each lesson
- Collect all approvals before making any file changes
- Always show final summary before applying changes

---

## Examples

### Good Lesson Extraction

**Conversation snippet:**
```
User: The heredoc isn't working in the justfile
Agent: Let me try using printf instead of a heredoc
[fixes the issue]
```

**Extracted Lesson:**
```
### 2024-12-31 - Justfile Heredoc Parsing Issue

When using bash scripts inside justfile recipes, nested heredocs cause parsing errors because justfile attempts to parse the heredoc content as justfile syntax before passing it to bash. This manifests as "Unknown start of token" errors pointing to bash syntax elements.

Solution: Use `printf '%s\n'` with escaped strings instead of heredocs when generating multi-line scripts inside justfile recipes. This avoids the parser conflict while maintaining the same functionality.
```

### Bad Lesson Extraction

❌ "Changed the heredoc to printf" - Too vague, no context
❌ "Here's the working code: [code]" - This is implementation, not a lesson
❌ "Fixed the bug" - No information about what was learned

---

## What NOT to Extract

- Routine operations (running commands, installing packages)
- Implementation details (specific code solutions)
- Expected behavior (things working as intended)
- Temporary experiments that didn't lead to insights
- Personal preferences without technical reasoning

## What TO Extract

- Unexpected failures and their root causes
- Discoveries about how tools/systems actually work
- Mistakes made and how they were identified
- Better approaches discovered during implementation
- Important decisions and the reasoning behind them
- Edge cases or limitations discovered
- Workarounds for known issues and why they're needed

# Command: Review Tasks

You are reviewing existing task files to ensure they follow proper requirements, identify issues, and suggest improvements.

## Purpose

Task files can degrade over time - they might contain code snippets, become too verbose, try to do too much, or violate the task specification format. This command audits task files and recommends corrections.

## Task File Requirements

A proper task file should:

✅ **HAVE:**
- YAML frontmatter with task_id, title, project, created date
- Summary (1 paragraph, describes WHAT and WHY)
- Goals (3 specific, measurable outcomes)
- File System Diff (tree structure showing expected changes)
- Lessons Learned section (can be empty or populated)
- Validation section (commands that must succeed)

❌ **NOT HAVE:**
- Code snippets or implementation details
- Step-by-step implementation instructions
- More than 3-4 goals (too broad)
- Verbose descriptions (should be concise)
- Mixed concerns (doing multiple unrelated things)

## Instructions

**CRITICAL RULES:**
1. **SCAN ALL TASKS** - Review all task files in `.context/tasks/`
2. **IDENTIFY ISSUES** - Find violations of task requirements
3. **ONE-BY-ONE RECOMMENDATIONS** - Present each issue individually with recommended fixes
4. **TASK SPLITTING** - Identify tasks that are too broad and suggest splitting them
5. **FINAL SUMMARY** - Show all approved changes for final confirmation before applying

### Workflow:

1. **Scan Task Files**:
   - Read all task files from `.context/tasks/`
   - For each task, check for:
     - **Format violations**: Missing sections, incorrect structure
     - **Code snippets**: Any code blocks in Summary, Goals, or other sections (except File System Diff)
     - **Verbosity**: Overly detailed descriptions, implementation steps
     - **Scope creep**: Too many goals (>4), mixed concerns, unrelated objectives
     - **Invalid content**: How-to instructions, implementation details instead of outcomes
   - Track all issues found across all tasks

2. **Categorize Issues**:
   - Group issues by type:
     - **Critical**: Code snippets, missing required sections
     - **Important**: Scope too broad, format violations
     - **Minor**: Verbose descriptions, too many goals
   - Prioritize critical issues first

3. **Check for Task Splitting Candidates**:
   - Identify tasks that are doing too much:
     - More than 4 goals
     - Goals that are unrelated to each other
     - File System Diff spanning too many unrelated areas
     - Summary trying to describe multiple distinct features
   - For each splitting candidate:
     - Analyze the goals and identify logical groupings
     - Propose how to split into multiple focused tasks
     - Suggest names for the split tasks

4. **Present Issues One-by-One** (Use Standardized Format):
   - For each issue found, present it individually:
     ```
     **Issue [N/TOTAL]**: TASK_XXX-description.md - <Issue Type>
     
     **What I noticed:** <Describe the specific issue found>
     
     **Example from task:**
     ```
     <Show the problematic content>
     ```
     
     **Recommended:** Option A - <Brief description of recommended fix>
     
     | Option | Description |
     |--------|-------------|
     | A | <Recommended fix> |
     | B | Skip - Leave as is |
     | C | Different fix - I'll specify |
     | Short | Provide custom action (≤5 words) |
     
     **Proposed fix:**
     ```
     <Show what the section will look like after the fix>
     ```
     
     You can reply with: "A"/"yes" to accept, "B" to skip, "C" to provide different fix, or your custom action.
     ```
   - Wait for user response for EACH issue
   - Track all approved fixes

5. **Present Task Splitting Recommendations**:
   - If tasks need splitting, present each one:
     ```
     **Recommendation [N/TOTAL]**: Split TASK_XXX - Task Too Broad
     
     **What I noticed:** This task has N goals covering multiple unrelated features: <list them>
     
     **Recommended:** Option A - Split into M focused tasks
     
     | Option | Description |
     |--------|-------------|
     | A | Split into M tasks - <list proposed task names> |
     | B | Keep as one task - Reduce goals instead |
     | C | Different approach - I'll specify |
     | Short | Provide custom action (≤5 words) |
     
     **Proposed split:**
     
     1. **TASK_XXX-1: <Name>**
        - Goals: <list goals>
     
     2. **TASK_XXX-2: <Name>**
        - Goals: <list goals>
     
     You can reply with: "A" to split, "B" to keep as one, or "C" for different approach.
     ```
   - Wait for user response
   - If user approves split, mark for task creation

6. **Final Summary & Confirmation**:
   - After ALL issues have been reviewed, present summary:
     ```
     **Summary of Changes:**
     
     I will make the following changes:
     
     **Files to Update:**
     1. TASK_XXX-description.md
        - Remove code snippets from Summary section
        - Reduce goals from 6 to 3
     
     2. TASK_YYY-other-task.md
        - Fix missing File System Diff section
        - Remove implementation steps
     
     **Tasks to Split:**
     1. TASK_ZZZ-big-task.md → Split into:
        - TASK_ZZZ-1-focused-task-a.md
        - TASK_ZZZ-2-focused-task-b.md
     
     **Tasks to Create:**
     - N new task files from splits
     
     **Proceed with these changes?**
     
     | Option | Description |
     |--------|-------------|
     | A | Yes - Apply all changes |
     | B | No - Cancel all changes |
     | C | Review - Show me specific changes again |
     
     Reply with: "A"/"yes" to proceed, "B"/"no" to cancel, or "C" to review.
     ```

7. **Apply Changes**:
   - If user approves (Option A or "yes"):
     - **For file updates**:
       - Read each task file
       - Apply approved corrections
       - Maintain proper formatting
       - Save updated file
     - **For task splits**:
       - Create new task files with auto-incremented IDs
       - Distribute goals/content appropriately
       - Update original task with reference to split tasks
       - Or archive original task (ask user preference)
     - Confirm all changes made
   - If user cancels (Option B or "no"):
     - Do not modify any files

8. **Completion**:
   - Show summary of what was done:
     ```
     ✅ Task review complete:
     
     **Updated:**
     - N task files corrected
     - M issues fixed
     
     **Created:**
     - X new focused task files from splits
     
     **Summary:**
     All tasks now comply with task specification requirements.
     Next: Run 'cdd' to see updated task list.
     ```

---

## Appendix A: Issue Detection Rules

### Critical Issues (Must Fix)

1. **Code Snippets in Wrong Sections**
   - Code blocks in Summary, Goals, or Lessons Learned
   - Implementation details instead of outcomes
   - **Exception**: File System Diff should have code-like tree structure

2. **Missing Required Sections**
   - No Summary
   - No Goals
   - No Validation
   - No File System Diff

3. **Invalid Format**
   - Missing or incorrect YAML frontmatter
   - Incorrect file naming (not TASK_XXX-description.md)

### Important Issues (Should Fix)

1. **Scope Too Broad**
   - More than 4 goals
   - Goals that are unrelated to each other
   - Trying to accomplish multiple distinct features

2. **Implementation Instead of Specification**
   - Step-by-step instructions ("First do X, then Y, then Z")
   - "How-to" content instead of "what" and "why"
   - Detailed technical implementation instead of outcomes

3. **Format Violations**
   - Goals not in bullet list format
   - Validation commands not in code block or bullet list
   - File System Diff not showing tree structure

### Minor Issues (Nice to Fix)

1. **Verbosity**
   - Summary longer than 2-3 sentences
   - Goals with too much detail
   - Overly detailed descriptions

2. **Too Many Goals**
   - 4-5 goals (consider reducing or splitting)
   - Redundant goals

3. **Unclear Validation**
   - Vague validation commands
   - Missing common validations (just dev, just test)

---

## Appendix B: Task Splitting Guidelines

### When to Split a Task

Split if:
- **6+ goals** - Too many objectives
- **Unrelated goals** - Goals cover different features/areas
- **Mixed concerns** - Frontend + Backend + Database in one task
- **File System Diff too large** - Changes span many unrelated directories
- **Summary describes multiple features** - Can't be summarized in one clear sentence

### How to Split

1. **Identify logical groupings**:
   - Group related goals together
   - Separate by layer (frontend, backend, database)
   - Separate by feature (authentication, authorization, session management)

2. **Propose focused tasks**:
   - Each split task should have 2-3 goals
   - Each should have clear, single focus
   - Each should be independently implementable (when possible)

3. **Handle dependencies**:
   - If splits have dependencies, note them in the task
   - Recommend implementation order
   - Add dependencies to Validation section

### Naming Split Tasks

- Original: `TASK_005-user-management-system.md`
- Split 1: `TASK_005-user-authentication.md`
- Split 2: `TASK_006-user-authorization.md`
- Split 3: `TASK_007-user-session-management.md`

---

## Appendix C: Example Issue Detection

### Example 1: Code Snippet in Summary

**Issue:**
```markdown
## Summary

This task implements user authentication using JWT tokens:

\`\`\`rust
pub struct User {
    id: Uuid,
    email: String,
}
\`\`\`
```

**Fix:**
```markdown
## Summary

This task implements JWT-based user authentication to secure API endpoints and manage user sessions.
```

### Example 2: Task Too Broad

**Issue:**
```markdown
## Goals

- Implement user authentication with JWT
- Add user registration flow
- Create admin dashboard
- Set up email notifications
- Implement password reset
- Add OAuth integration
```

**Fix:** Split into 3 tasks:
- TASK_XXX-user-authentication.md (JWT + login)
- TASK_YYY-user-registration.md (registration + email + password reset)
- TASK_ZZZ-admin-dashboard.md (admin UI + OAuth)

### Example 3: Implementation Steps Instead of Outcomes

**Issue:**
```markdown
## Goals

- First, create the database schema
- Then, implement the API endpoints
- After that, add frontend components
- Finally, write tests
```

**Fix:**
```markdown
## Goals

- User authentication system with JWT token management
- Secure API endpoints requiring authentication
- Frontend login/logout interface with session persistence
```

---

## Best Practices

- Review ALL tasks, not just recent ones
- Prioritize critical issues over minor ones
- For task splits, suggest sensible groupings based on actual goals
- Show examples of problematic content when presenting issues
- Always show proposed fix so user knows exactly what will change
- Be conservative with splits - only suggest when task is clearly too broad
- Allow user to skip fixes if they disagree
- Collect all approvals before making ANY changes to files

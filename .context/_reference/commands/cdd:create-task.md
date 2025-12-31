# Command: Create Task

You are creating a new task specification following the TASK.md template format.

## Template Location

`.context/_reference/templates/TASK.md`

## Instructions

1. **Understand the Goal**: First, ask the user what they are trying to achieve:
   - "What are you trying to accomplish with this task?"
   - Listen to their response to understand the context

2. **Identify Applicable Rules**: Based on the user's goal, analyze which rules from `.context/_reference/rules/` should apply:
   - Check available rule files (e.g., `rs.md`, `justfiles.md`, etc.)
   - Make an informed assumption about which rules are relevant
   - Example: If building Rust code, `rs.md` rules apply
   - Example: If creating build commands, `justfiles.md` rules apply
   - Inform the user which rules you're applying and why

3. **Gather Task Information**: Ask the user for the following information using the standardized question format (see Appendix A):
   - **Task ID**: What should the task ID be? (e.g., TASK_001, TASK_002)
   - **Title**: What is the task title?
   - **Project**: Which project does this belong to? (e.g., PROJECT_XXX, NEXUS_001, APP_001)
   - **Summary**: One paragraph describing what this task accomplishes and why it matters
   - **Goals**: 3 specific, measurable outcomes

4. **Generate Task File**: Create a task file with:
   - **File Name**: `TASK_XXX-brief-description.md` (e.g., `TASK_001-add-user-auth.md`)
   - Proper frontmatter with task_id, title, project, created date (today)
   - Summary section
   - Goals section (3 goals)
   - File System Diff (showing expected file changes with tree structure)
   - Lessons Learned section (EMPTY - will be filled after implementation)
   - Validation section (must include `just dev` and `just test` commands)

5. **Save Location**: Save the file to the appropriate location based on the project structure in `.context/`
   - Pattern: `.context/{project-folder}/{project-id}/TASK_{id}-{slug}.md`
   - Example: `.context/nexus-core/CORE_001-backend-crate/TASK_005-implement-auth.md`
   - If no complex project structure exists, save to: `.context/tasks/TASK_{id}-{slug}.md`

---

## Appendix A: Standardized Question-Asking Format

When asking users questions, use this standardized format:

```
**Question [N]**: <question text>

**Recommended:** Option [X] - <1-2 sentence reasoning why this is best>

| Option | Description |
|--------|-------------|
| A | <description> |
| B | <description> |
| C | <description> |
| Short | Provide different answer (≤5 words) |

You can reply with: option letter (e.g., "B"), "yes"/"recommended" to accept, or your own short answer.
```

**Response Parsing Rules:**

- `"yes"`, `"recommended"`, `"suggested"` → Use recommended option
- Option letter (A, B, C, etc.) → Use that option
- Short answer → Use that answer
- `"done"`, `"good"`, `"no more"` → Stop asking questions

**Best Practices:**

- Ask as many questions as needed to gather complete information
- Provide smart defaults as "Recommended" with reasoning
- Offer clear options in table format
- Allow short custom answers for flexibility
- Collect all answers before taking action
- Stop early if user says "done", "good", or "no more"

This format ensures consistent, efficient user interaction across all creator and interview-style skills
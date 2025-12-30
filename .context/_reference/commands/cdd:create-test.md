# Command: Create Test

You are creating a new test task specification using the unified TASK.md template format.

## Template Location

`.context/context_driven_development/templates/TASK.md`

## Instructions

1. Read the TASK.md template to understand the structure
2. Ask the user for the following information using the standardized question format (see Appendix A):
   - **Task ID**: What should the task ID be? (e.g., TASK_001)
   - **Title**: What is being tested? (e.g., "Test Authentication System")
   - **Project**: Which project does this belong to? (e.g., CORE_001-backend-crate)
   - **Summary**: What is being validated and the testing strategy
   - **Goals**: 3 specific testing outcomes (e.g., "All edge cases covered", "100% code coverage for auth module")

3. Generate a task file with:
   - Proper frontmatter with task_id, title, project, created date (today)
   - Summary section (describe what is being tested and why)
   - Goals section (3 testing-focused goals)
   - File System Diff (showing test files being added, e.g., `+ tests/test_auth.rs`)
   - Lessons Learned section (EMPTY - will be filled after implementation)
   - Validation section with:
     - `just test` - All tests pass (REQUIRED)
     - `just dev` - Development environment still works
     - Additional test commands as needed

4. Save the file to the appropriate location based on the project structure in `.context/`
   - Pattern: `.context/{project-folder}/{project-id}/TASK_{id}-{slug}.md`
   - Example: `.context/nexus-core/CORE_001-backend-crate/TASK_006-test-authentication.md`

## Key Differences for Test Tasks

- **Summary**: Focus on what is being validated and the testing strategy
- **Goals**: Testing-focused outcomes (coverage, edge cases, integration points)
- **File System Diff**: Show test files being added (e.g., `+ tests/`, `+ **/*_test.rs`)
- **Validation**: MUST include `just test` as primary validation command

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

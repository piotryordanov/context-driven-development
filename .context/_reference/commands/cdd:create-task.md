# Command: Create Task

You are creating a new task specification following the TASK.md template format.

## Task Template (Embedded for Speed)

**Note:** This is embedded here so you don't need to read the file from disk. Use this template directly.

```markdown
---
task_id: TASK_XXX
title: Task Title
project: PROJECT_XXX
created: "YYYY-MM-DD"
updated: "YYYY-MM-DD"  # Optional: Last modification date
---

<!-- 
FILE NAMING: The task file should be named: TASK_XXX-brief-description.md
Example: TASK_001-add-user-authentication.md, TASK_042-fix-login-bug.md

NOTE: This is a task specification, not implementation.
- Describe WHAT to build, not HOW
- NO code snippets - code belongs in the codebase
- Focus on goals, outcomes, and lessons learned
-->

# TASK_XXX: <Title>

## Summary

<One paragraph describing what this task accomplishes and why it matters. For test tasks, describe what is being validated and the testing strategy.>

## Goals

- <Goal 1: What specific outcome this task achieves>
- <Goal 2: What component/feature is created or modified>
- <Goal 3: What success looks like when complete>

## File System Diff

<Expected file tree changes after completing this task. Show the tree structure with +/- indicators. The example below is just for illustration - create as many entries as needed for your task.>

\`\`\`
project/
├── src/
│   ├── + new_module.rs
│   ├── ~ modified_file.rs
│   └── components/
│       └── + new_component.rs
├── tests/
│   └── + test_new_feature.rs
└── - old_deprecated_file.rs
\`\`\`

## Lessons Learned

<This section should be EMPTY when the task is first created. Only add content here after attempting to implement the task and encountering issues, failures, or learning important lessons. Document what was learned and why it matters for future reference.>

## Validation

<Shell commands that MUST succeed after implementation. At minimum, ensure `just dev` works without breaking. For test tasks, ensure `just test` passes. The examples below are common validations - add as many commands as needed for your task.>

- `just dev` - Development environment runs without errors
- `just test` - All tests pass (required for test tasks)
- `<additional command>` - <What it validates>
```

## Instructions

**CRITICAL RULES:**
1. **NEVER ASK FOR TASK ID** - You determine it automatically by scanning `.context/tasks/`
2. **START WITH UNDERSTANDING** - First message should ask what the task is about, NOT jump into questions
3. **CONVERSATION FIRST** - Have a conversation to understand the task before asking structured questions

### Workflow:

1. **First Message - Ask What The Task Is About**:
   - Your FIRST message to the user must be: "What are you trying to accomplish with this task?"
   - Read what the user wrote when invoking the command
   - If they already described their goal clearly, acknowledge it: "I understand you want to [paraphrase their goal]. Let me ask a few clarifying questions..."
   - If unclear or insufficient detail, ask: "Can you tell me more about what you're trying to accomplish?"
   - Have a conversation to fully understand the context and requirements
   - DO NOT ask structured questions until you understand the task

2. **Auto-determine Task ID (SILENT - No Question)**:
   - Check `.context/tasks/` directory for existing tasks
   - Find the highest TASK_XXX number (e.g., if TASK_002 exists, next is TASK_003)
   - If no tasks exist, start with TASK_001
   - **ABSOLUTELY NEVER ASK THE USER FOR THE TASK ID** - This is 100% automatic
   - Simply use the auto-incremented ID when creating the file
   - You can mention it: "I'll create this as TASK_003" but never ask about it

3. **Identify Applicable Rules**:
   - Based on your understanding of the task, analyze which rules from `.context/_reference/rules/` apply
   - Check available rule files (e.g., `rs.md`, `justfiles.md`, etc.)
   - Example: If building Rust code, `rs.md` rules apply
   - Example: If creating build commands, `justfiles.md` rules apply
   - Inform the user which rules you're applying and why

4. **Gather Remaining Information** (ONLY if needed):
   - Use the standardized question format (see Appendix A) ONLY for information you couldn't infer
   - Questions you may need to ask:
     - **Title**: What is the task title? (If not clear from conversation)
     - **Project**: Which project does this belong to? (Default: use repo name or "MAIN")
     - **Summary**: One paragraph describing what this task accomplishes (If you can write it based on conversation, do so)
     - **Goals**: 3 specific, measurable outcomes (If you can infer from conversation, propose them)
   - Keep questions minimal - infer as much as possible from the initial conversation

5. **Generate Task File**:
   - **File Name**: `TASK_XXX-brief-description.md` (e.g., `TASK_001-add-user-auth.md`)
   - Proper frontmatter with task_id (auto-incremented), title, project, created date (today)
   - Summary section
   - Goals section (3 goals)
   - File System Diff (showing expected file changes with tree structure)
   - Lessons Learned section (EMPTY - will be filled after implementation)
   - Validation section (must include `just dev` and `just test` commands)

6. **Save Location**: Save all task files to `.context/tasks/`
   - Pattern: `.context/tasks/TASK_{id}-{slug}.md`
   - Example: `.context/tasks/TASK_001-add-user-auth.md`
   - The `.context/tasks/` directory is created during CDD initialization

---

## Appendix A: Standardized Question-Asking Format

When asking users questions, use this standardized format:

```
**Question [N/TOTAL]**: <question text>

**Recommended:** Option [X] - <1-2 sentence reasoning why this is best>

| Option | Description |
|--------|-------------|
| A | <description> |
| B | <description> |
| C | <description> |
| Short | Provide different answer (≤5 words) |

You can reply with: option letter (e.g., "B"), "yes"/"recommended" to accept, or your own short answer.
```

**Examples:**
- `**Question [1/3]**: What is the task title?`
- `**Question [2/3]**: Which project does this belong to?`
- `**Question [3/3]**: Does this summary capture the task correctly: "<proposed summary>"?`

**Response Parsing Rules:**

- `"yes"`, `"recommended"`, `"suggested"` → Use recommended option
- Option letter (A, B, C, etc.) → Use that option
- Short answer → Use that answer
- `"done"`, `"good"`, `"no more"` → Stop asking questions

**Best Practices:**

- Always show question count in format `[N/TOTAL]` (e.g., `[1/4]`, `[2/4]`)
- Calculate TOTAL upfront based on required information
- Ask as many questions as needed to gather complete information
- Provide smart defaults as "Recommended" with reasoning
- Offer clear options in table format
- Allow short custom answers for flexibility
- Collect all answers before taking action
- Stop early if user says "done", "good", or "no more"

This format ensures consistent, efficient user interaction across all creator and interview-style skills
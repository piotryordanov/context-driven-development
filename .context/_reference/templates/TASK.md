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

```
project/
├── src/
│   ├── + new_module.rs
│   ├── ~ modified_file.rs
│   └── components/
│       └── + new_component.rs
├── tests/
│   └── + test_new_feature.rs
└── - old_deprecated_file.rs
```

## Lessons Learned

<This section should be EMPTY when the task is first created. Only add content here after attempting to implement the task and encountering issues, failures, or learning important lessons. Document what was learned and why it matters for future reference.>

## Validation

<Shell commands that MUST succeed after implementation. At minimum, ensure `just dev` works without breaking. For test tasks, ensure `just test` passes. The examples below are common validations - add as many commands as needed for your task.>

- `just dev` - Development environment runs without errors
- `just test` - All tests pass (required for test tasks)
- `<additional command>` - <What it validates>

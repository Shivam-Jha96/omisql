---
name: sync-docs
description: >-
  Use this skill when the user asks to update project documentation (such as implementation_plan.md) to ensure it accurately reflects the latest repository state after new features are implemented.
---

# Documentation Sync Skill

This skill provides the standard operating procedure for updating the project's living documentation (e.g., `implementation_plan.md` or `docs/`) to reflect newly implemented features, ensuring all docs are kept perfectly in sync with the repository state.

## Steps

1. **Verify Current State**:
   - Run `git status` and `git log -n 3` to understand what changes were recently pushed or committed.
   - Review the specific code files that were updated to confirm what features were implemented.

2. **Locate the Documentation**:
   - Primarily, look for `implementation_plan.md` in the repository root.
   - Look for other relevant docs in the `docs/` folder.

3. **Update the Documentation**:
   - Add a summary of the completed work. If a specific phase (e.g., Phase 6, Phase 7) was completed, mark it as `[COMPLETED]` in the document headers.
   - List the newly created `[NEW]` and modified `[MODIFY]` files and explain what was changed in them.
   - Be extremely careful with text encodings and Markdown formatting when updating files programmatically. Ensure no data is accidentally truncated or corrupted.

4. **Commit and Push**:
   - Run `git status` to ensure only the intended documentation files are modified.
   - Commit the changes with an appropriate conventional commit message (e.g., `docs: update implementation plan with recent completions`).
   - Push the changes to the current branch using `git push origin HEAD`.

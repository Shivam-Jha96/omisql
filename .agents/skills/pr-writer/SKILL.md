---
name: pr-writer
description: >-
  Use this skill when the user asks to write, draft, or generate a Pull Request (PR) description for their GitHub repository.
---

# PR Writer Skill

This skill provides the standard operating procedure for generating highly structured, comprehensive GitHub Pull Request descriptions.

## Steps

1. **Analyze the Changes**:
   - Run `git log -n 10` or `git log main..HEAD` (or `master..HEAD`) to gather recent commit messages.
   - Run `git diff main...HEAD --stat` to see which files were changed.
   - Run `git diff main...HEAD` to review the actual code changes if necessary (use pagination or limits if the diff is massive).

2. **Cross-Reference Documentation**:
   - Check if there are updates in `implementation_plan.md` or `docs/` that provide context on the high-level objective being solved.

3. **Draft the PR Description**:
   Create an artifact (e.g., `pr_description.md`) using the following structure:
   
   ```markdown
   # Pull Request: [Feature/Bugfix Name]
   
   ## 🎯 Objective
   [Brief 1-3 sentence summary of what this PR accomplishes and why it is needed.]
   
   ## 🚀 Key Features
   [Break down the changes logically, grouping by component, file, or architectural tier. Use bullet points.]
   - **[Component A]**: [Description]
   - **[Component B]**: [Description]
   
   ## 🧪 Testing & Validation
   [Explain how you verified that these changes work. Mention any `cargo check`, `cargo test`, or manual verification steps you performed.]
   
   ## 📝 Reviewer Checklist
   - [ ] Code compiles successfully.
   - [ ] Documentation has been updated to reflect changes.
   - [ ] No large binaries or unneeded files were accidentally committed.
   ```

4. **Deliver to User**:
   - Save the markdown in the Antigravity artifact format.
   - Advise the user to copy-paste the artifact contents into GitHub.
   - Optionally, check if the GitHub CLI (`gh`) is installed (`gh --version`). If it is, offer to automate opening the PR using `gh pr create --body-file <path>`.

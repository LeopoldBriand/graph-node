# Contribution Rules & Guideline

- [Contribution Rules & Guideline](#contribution-rules--guideline)
  - [Why](#why)
  - [Project Management](#project-management)
    - [Definitions](#definitions)
    - [Organisation](#organisation)
    - [How to add an issue to a project](#how-to-add-an-issue-to-a-project)
    - [Labels management](#labels-management)
    - [Issues & Pull Requests](#issues--pull-requests)
    - [Branch name](#branch-name)
  - [Git Flow](#git-flow)
    - [Commit convention](#commit-convention)
      - [Header](#header)
      - [Body](#body)
    - [Merging policies](#merging-policies)
      - [Merging branch in master](#merging-branch-in-master)
      - [Merging not allows](#merging-not-allows)
    - [Avoid](#avoid)
    - [Pull Request workflow](#pull-request-workflow)
  - [Git tips](#git-tips)
    - [How to edit an old commit](#how-to-edit-an-old-commit)
    - [Git reflog save the Queen](#git-reflog-save-the-queen)
    - [Other usefull links about Git](#other-usefull-links-about-git)

## Why

To ensure seamless flow, quality code and documention. This will also enable
other to take over your work.

## Project Management

### How to add an issue to a project

Read Github
[documentation](https://help.github.com/en/github/managing-your-work-on-github/adding-issues-and-pull-requests-to-a-project-board)

**IMPORTANT NOTE:**

> please take good care about how you add issues to projects. Prefer only adding
> tracking/follow up issues of your own on-going work. But avoid for not
> prioritized work.

### Labels management

- Don't create labels for everything
- All label are lowercase name
- Status:
  - Issues:
    - `wip`: open your PR as a Draft
    - `on hold`: stopped for any reason => add comment when setting this label
      so we can remember why
    - `blocked`: blocked for any reason => add comment when setting this label
      so we can remember why
  - Pull requests:
    - `spike`
    - `do not merge`
- Types:
  - `bug`: for ... bugs (things that DO NOT work)
  - `improvement`: not a real bug but must be better (basically it works)
  - `enhancement`: new feature, better UX, better dev experience ...
- Others:
  - `help wanted`

### Issues & Pull Requests

- Title:
  - Simple description of the bug, the feature request / enhancement, spec/epic
    subject ...
  - No state (wip, bug...) which are covered by labels
- Reference issues in PR
  - example: in the PR message insert `fix #102` or `close #102` will auto close the issue 102 when the PR is merged

### Branch name

- No special rules

## Git Flow

### Commit convention

Based on
[Angular's guidelines](https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#-commit-message-guidelines)

#### Header

`{type}({scope}): {change description}`

Must be one of the following:

- **ci**: Changes to our CI configuration files and scripts (example scopes:
  Travis, Circle, BrowserStack, SauceLabs)
- **docs**: Documentation only changes
- **feat**: A new feature
- **fix**: A bug fix
- **perf**: A code change that improves performance
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **style**: Changes that do not affect the meaning of the code (white-space,
  formatting, missing semi-colons, etc)
- **test**: Adding missing tests or correcting existing tests

- Scope:

  - one of the core modules (**builder**, **node** ...) 

#### Body

To use when discussed and agreed upfront:

- add "Breaking Changes" to warn about any breaking changes

### Merging policies

- merge is allowed on PR with signed merge commit by github. Only when closing
  PR.
- always use rebase.

#### Merging branch in master

- use "squash and merge" if there some ugly commits in history
- use "rebase and merge" if commits history is clean

#### Merging not allows

- merge branch1 in branch1

### Pull Request workflow

- name your PR with a short and descriptive title
- assign PR to yourself
- assign reviewers
- describe your changes
- Mark fix issue `fix #{{ issue number}}`

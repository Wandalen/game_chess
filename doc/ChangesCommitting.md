## How to commit changes

At this stage we assume that you successfully [built](./Build.md) the project and have a [ticket](./TicketsLabeling.md) to work on.

**Attention**. All actions with [upstream repository](https://github.com/Wandalen/game_chess.git) should be performed using branch `alpha`. Pull changes from branch `alpha` and open pull requests into branch `alpha`. Upstream branches `beta` and `main` update automatically.

## Steps

- Create a branch for your ticket: `git checkout -b [name] alpha`
- Make changes
- Make a commit and push changes
- Synchronize branch with upstream branch `alpha`
- Open [pull request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request-from-a-fork) to branch `alpha`. Notice, pull request should be opened to branch `alpha` not to `main` or `beta`
- Watch CI results
- Get Dmytro’s review
- Make requested changes
- Get your pull request merged in

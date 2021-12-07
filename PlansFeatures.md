# Comparison of features

- [Official comparison between plans.](https://github.com/pricing)

Summary : the plan `Team` gives minimal benefits against plan `Free` for GitHubAction minutes, the plan `Enterprise` has optimal features.

- [Number of parallel jobs.](https://docs.github.com/en/actions/learn-github-actions/usage-limits-billing-and-administration#usage-limits)
| GitHub plan | Total concurrent jobs | Maximum concurrent macOS jobs |
|-------------|-----------------------|-------------------------------|
| Free        | 20                    | 5                             |
| Pro         | 40                    | 5                             |
| Team        | 60                    | 5                             |
| Enterprise  | 180                   | 50                            |

Summary : the plan `Team` gives 3x concurrent jobs against plan `Free`, the plan `Enterprise` 9x concurrent jobs against plan `Free`. The main advantage of plan `Enterprise` is number of concurrent jobs on MacOS.

- [Github runners ( default feature ) for all plans have same characteristics.](https://docs.github.com/en/actions/using-github-hosted-runners/about-github-hosted-runners#supported-runners-and-hardware-resources)

Hardware specification for Windows and Linux virtual machines:

-- 2-core CPU

-- 7 GB of RAM memory

-- 14 GB of SSD disk space

Hardware specification for macOS virtual machines:

-- 3-core CPU

-- 14 GB of RAM memory

-- 14 GB of SSD disk space

- Workflows have no daily limits for GitHubAction minutes.

- The spent minutes calculates using multipliers :

| Operating system | Minute multiplier |
|------------------|-------------------|
| Linux            | 1                 |
| macOS            | 10                |
| Windows          | 2                 |

[More about limits and calculations.](https://docs.github.com/en/billing/managing-billing-for-github-actions/about-billing-for-github-actions)

- If you've maxed out the Actions minutes limit ( or another resource ) for your billing plan for the current billing period and you'd like to continue using Actions before the start of your next billing period, an org owner or billing manager can set a spending limit ( or unlimited spending ) for Actions overages on the billing page [( help page with information ).](https://help.github.com/en/github/setting-up-and-managing-billing-and-payments-on-github/managing-your-spending-limit-for-github-actions)

- [The organization paid for each member of private repository ( maintainer or collaborator ).](https://docs.github.com/en/billing/managing-billing-for-your-github-account/about-per-user-pricing)

Summary : organization can have a single member ( owner ) with private repositories if this user will manually reflect changes from some repository to the owned repository.

# EVIDENCE: which clean open PRs would run the sound-device test that hung locally

> **EVIDENCE / investigation note. Not an application fix and not a proposal.** Written by Ian Worley's AI agent
> via T3 Code at Ian's request, published only to the fork `IanWorley/horizon`. Nothing was written to
> `horizon-rust/horizon` (no push, PR, issue or comment).

- Snapshot taken: **2026-11-06 14:11 CST** (America/Chicago).
- Upstream `main` at snapshot: **`41f822b659789780f58d32c63ddfb6887ea1b8bf`** (`git ls-remote origin refs/heads/main` before analysis and again before
  publishing). The recorded SHA `41f822b659789780f58d32c63ddfb6887ea1b8bf` is the same commit: **main did not move.** `41f822b6..origin/main` = 0 commits.
- Related: issue [#11342](https://github.com/horizon-rust/horizon/issues/11342), the local hangs on
  [#10194](https://github.com/horizon-rust/horizon/pull/10194) and [#10195](https://github.com/horizon-rust/horizon/pull/10195).
- Test in question: `options_ui::tests::sound_device_button_in_sound_page_and_cycles`, [`crates/horizon-client-render/src/options_ui.rs:7602-7625`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-render/src/options_ui.rs#L7602-L7625).
- No tests were run for this note. Everything below comes from git objects, `Cargo.toml` manifests, workflow files and
  the GitHub REST API (read-only).

## 1. Answer

Primary set: open PRs on `horizon-rust/horizon` with `draft == false`, `mergeable == true` and
`mergeable_state == "clean"`, excluding #10194 and #10195. **114 PRs.** (#10194 and #10195 are currently
`mergeable_state == "unstable"`, so the clean filter excludes them anyway. They appear only as contrast in §9.)

| Result for the 114 clean PRs | Count |
|---|---|
| Local `pr_ready.py --quick` at the PR head **runs the test with the `audio` feature on** (real `cpal` enumeration) | **25** |
| Local `--quick` runs the test with `audio` **off** (stub `output_devices()` returns `[]`, no `cpal`) | 5 |
| Local `--quick` does **not** run the test (render's unit tests not selected) | 67 (of which 52 build render only as a dependency) |
| Local `--quick` does **not** run the test (no workspace-Rust path, so no cargo test step) | 17 |
| GitHub PR CI `checks / verify` **runs** the test (Linux, `cargo test --workspace`, `audio` on) | **109** |
| GitHub PR CI does **not** run it (docs-only change: Rust steps skipped) | 5 |

- **Rebasing changes no conclusion.** Every head examined (all 197 clean+unstable heads,
  #10194/#10195 included) already contains the test with a body identical to main's (hash of the function text), and
  every head's merge base already contains the commit that brought the test to main. Every head's copy of the
  `pr_ready.py` selection code is textually identical to main's. Rebasing onto `41f822b6` (= live main) yields the
  same verdict and the same crate list for every PR.
- **"Runs the test" is not "hits the hang".** Section 4 explains why. Even in the audio-on group, the local result
  depends on the OS audio stack and on `HORIZON_NO_AUDIO` / `HORIZON_AUDIO` / `WAYLAND_DISPLAY` in the test
  process's environment. A hang was observed on one Mac. Nothing here shows a hang on Linux, on CI, or on every Mac.

### Would run locally with `audio` ON (25)
[#10039](https://github.com/horizon-rust/horizon/pull/10039), [#10040](https://github.com/horizon-rust/horizon/pull/10040), [#10168](https://github.com/horizon-rust/horizon/pull/10168), [#10197](https://github.com/horizon-rust/horizon/pull/10197), [#10245](https://github.com/horizon-rust/horizon/pull/10245), [#10537](https://github.com/horizon-rust/horizon/pull/10537), [#10559](https://github.com/horizon-rust/horizon/pull/10559), [#11010](https://github.com/horizon-rust/horizon/pull/11010), [#11014](https://github.com/horizon-rust/horizon/pull/11014), [#11019](https://github.com/horizon-rust/horizon/pull/11019), [#11028](https://github.com/horizon-rust/horizon/pull/11028), [#11051](https://github.com/horizon-rust/horizon/pull/11051), [#11062](https://github.com/horizon-rust/horizon/pull/11062), [#11070](https://github.com/horizon-rust/horizon/pull/11070), [#11076](https://github.com/horizon-rust/horizon/pull/11076), [#11090](https://github.com/horizon-rust/horizon/pull/11090), [#11092](https://github.com/horizon-rust/horizon/pull/11092), [#11098](https://github.com/horizon-rust/horizon/pull/11098), [#11099](https://github.com/horizon-rust/horizon/pull/11099), [#11111](https://github.com/horizon-rust/horizon/pull/11111), [#11113](https://github.com/horizon-rust/horizon/pull/11113), [#11115](https://github.com/horizon-rust/horizon/pull/11115), [#11116](https://github.com/horizon-rust/horizon/pull/11116), [#11117](https://github.com/horizon-rust/horizon/pull/11117), [#11118](https://github.com/horizon-rust/horizon/pull/11118)

### Would run locally with `audio` OFF (stub path, no CoreAudio from this test) (5)
[#10531](https://github.com/horizon-rust/horizon/pull/10531), [#10556](https://github.com/horizon-rust/horizon/pull/10556), [#11054](https://github.com/horizon-rust/horizon/pull/11054), [#11144](https://github.com/horizon-rust/horizon/pull/11144), [#11162](https://github.com/horizon-rust/horizon/pull/11162)

### Would not run locally: render not selected (67)
[#9566](https://github.com/horizon-rust/horizon/pull/9566), [#9952](https://github.com/horizon-rust/horizon/pull/9952), [#9953](https://github.com/horizon-rust/horizon/pull/9953), [#10086](https://github.com/horizon-rust/horizon/pull/10086), [#10096](https://github.com/horizon-rust/horizon/pull/10096), [#10099](https://github.com/horizon-rust/horizon/pull/10099), [#10133](https://github.com/horizon-rust/horizon/pull/10133), [#10177](https://github.com/horizon-rust/horizon/pull/10177), [#10208](https://github.com/horizon-rust/horizon/pull/10208), [#10240](https://github.com/horizon-rust/horizon/pull/10240), [#10242](https://github.com/horizon-rust/horizon/pull/10242), [#10365](https://github.com/horizon-rust/horizon/pull/10365), [#10373](https://github.com/horizon-rust/horizon/pull/10373), [#10527](https://github.com/horizon-rust/horizon/pull/10527), [#10542](https://github.com/horizon-rust/horizon/pull/10542), [#10546](https://github.com/horizon-rust/horizon/pull/10546), [#10551](https://github.com/horizon-rust/horizon/pull/10551), [#10552](https://github.com/horizon-rust/horizon/pull/10552), [#10553](https://github.com/horizon-rust/horizon/pull/10553), [#10554](https://github.com/horizon-rust/horizon/pull/10554), [#10564](https://github.com/horizon-rust/horizon/pull/10564), [#10565](https://github.com/horizon-rust/horizon/pull/10565), [#10570](https://github.com/horizon-rust/horizon/pull/10570), [#10571](https://github.com/horizon-rust/horizon/pull/10571), [#10572](https://github.com/horizon-rust/horizon/pull/10572), [#10703](https://github.com/horizon-rust/horizon/pull/10703), [#10837](https://github.com/horizon-rust/horizon/pull/10837), [#10908](https://github.com/horizon-rust/horizon/pull/10908), [#10967](https://github.com/horizon-rust/horizon/pull/10967), [#10996](https://github.com/horizon-rust/horizon/pull/10996), [#11007](https://github.com/horizon-rust/horizon/pull/11007), [#11009](https://github.com/horizon-rust/horizon/pull/11009), [#11016](https://github.com/horizon-rust/horizon/pull/11016), [#11017](https://github.com/horizon-rust/horizon/pull/11017), [#11022](https://github.com/horizon-rust/horizon/pull/11022), [#11023](https://github.com/horizon-rust/horizon/pull/11023), [#11030](https://github.com/horizon-rust/horizon/pull/11030), [#11032](https://github.com/horizon-rust/horizon/pull/11032), [#11033](https://github.com/horizon-rust/horizon/pull/11033), [#11036](https://github.com/horizon-rust/horizon/pull/11036), [#11043](https://github.com/horizon-rust/horizon/pull/11043), [#11044](https://github.com/horizon-rust/horizon/pull/11044), [#11046](https://github.com/horizon-rust/horizon/pull/11046), [#11049](https://github.com/horizon-rust/horizon/pull/11049), [#11056](https://github.com/horizon-rust/horizon/pull/11056), [#11057](https://github.com/horizon-rust/horizon/pull/11057), [#11065](https://github.com/horizon-rust/horizon/pull/11065), [#11067](https://github.com/horizon-rust/horizon/pull/11067), [#11068](https://github.com/horizon-rust/horizon/pull/11068), [#11071](https://github.com/horizon-rust/horizon/pull/11071), [#11083](https://github.com/horizon-rust/horizon/pull/11083), [#11100](https://github.com/horizon-rust/horizon/pull/11100), [#11107](https://github.com/horizon-rust/horizon/pull/11107), [#11108](https://github.com/horizon-rust/horizon/pull/11108), [#11114](https://github.com/horizon-rust/horizon/pull/11114), [#11122](https://github.com/horizon-rust/horizon/pull/11122), [#11127](https://github.com/horizon-rust/horizon/pull/11127), [#11129](https://github.com/horizon-rust/horizon/pull/11129), [#11133](https://github.com/horizon-rust/horizon/pull/11133), [#11135](https://github.com/horizon-rust/horizon/pull/11135), [#11136](https://github.com/horizon-rust/horizon/pull/11136), [#11139](https://github.com/horizon-rust/horizon/pull/11139), [#11152](https://github.com/horizon-rust/horizon/pull/11152), [#11153](https://github.com/horizon-rust/horizon/pull/11153), [#11155](https://github.com/horizon-rust/horizon/pull/11155), [#11157](https://github.com/horizon-rust/horizon/pull/11157), [#11159](https://github.com/horizon-rust/horizon/pull/11159)

### Would not run locally: no workspace Rust change (17)
[#10042](https://github.com/horizon-rust/horizon/pull/10042), [#10166](https://github.com/horizon-rust/horizon/pull/10166), [#10529](https://github.com/horizon-rust/horizon/pull/10529), [#10540](https://github.com/horizon-rust/horizon/pull/10540), [#11042](https://github.com/horizon-rust/horizon/pull/11042), [#11064](https://github.com/horizon-rust/horizon/pull/11064), [#11079](https://github.com/horizon-rust/horizon/pull/11079), [#11087](https://github.com/horizon-rust/horizon/pull/11087), [#11089](https://github.com/horizon-rust/horizon/pull/11089), [#11096](https://github.com/horizon-rust/horizon/pull/11096), [#11112](https://github.com/horizon-rust/horizon/pull/11112), [#11131](https://github.com/horizon-rust/horizon/pull/11131), [#11134](https://github.com/horizon-rust/horizon/pull/11134), [#11142](https://github.com/horizon-rust/horizon/pull/11142), [#11149](https://github.com/horizon-rust/horizon/pull/11149), [#11151](https://github.com/horizon-rust/horizon/pull/11151), [#11156](https://github.com/horizon-rust/horizon/pull/11156)

Of these 17, 12 still run the whole workspace's tests in GitHub CI (they change non-doc, non-crate paths such as
`tools/`, `tests/` or `.github/`): [#10166](https://github.com/horizon-rust/horizon/pull/10166), [#10529](https://github.com/horizon-rust/horizon/pull/10529), [#10540](https://github.com/horizon-rust/horizon/pull/10540), [#11042](https://github.com/horizon-rust/horizon/pull/11042), [#11064](https://github.com/horizon-rust/horizon/pull/11064), [#11079](https://github.com/horizon-rust/horizon/pull/11079), [#11087](https://github.com/horizon-rust/horizon/pull/11087), [#11096](https://github.com/horizon-rust/horizon/pull/11096), [#11112](https://github.com/horizon-rust/horizon/pull/11112), [#11131](https://github.com/horizon-rust/horizon/pull/11131), [#11142](https://github.com/horizon-rust/horizon/pull/11142), [#11151](https://github.com/horizon-rust/horizon/pull/11151).

### GitHub CI would not run it: docs-only (5)
[#10042](https://github.com/horizon-rust/horizon/pull/10042), [#11089](https://github.com/horizon-rust/horizon/pull/11089), [#11134](https://github.com/horizon-rust/horizon/pull/11134), [#11149](https://github.com/horizon-rust/horizon/pull/11149), [#11156](https://github.com/horizon-rust/horizon/pull/11156)

## 2. Enumeration and filter

`gh api --paginate 'repos/horizon-rust/horizon/pulls?state=open&per_page=100'` returned **269 open PRs**
(all with base `main`). The list endpoint has no `mergeable` field, so each PR was fetched with
`gh api repos/horizon-rust/horizon/pulls/<n>`. One fetch failed with HTTP 401 and was retried successfully. No PR
was left with `mergeable == null`.

| draft | mergeable | mergeable_state | PRs |
|---|---|---|---|
| False | True | `clean` | 114 |
| False | True | `unstable` | 83 |
| False | True | `blocked` | 34 |
| False | False | `dirty` | 25 |
| True | False | `dirty` | 6 |
| True | True | `clean` | 5 |
| True | True | `blocked` | 1 |
| True | True | `unstable` | 1 |

Filter: `draft == false && mergeable == true && mergeable_state == "clean"` gives 114. Excluding #10194/#10195 removes
nothing, because both are `unstable`. **Primary set: 114.** No PR had `mergeable_state == "behind"`.

Each head was fetched (`refs/pull/<n>/head`) and checked against the API's `head.sha`: all matched. "API base SHA" is
the API's `base.sha` field at snapshot time; it is not necessarily the base SHA of the PR's last CI run. "Merge base"
is `git merge-base <head> 41f822b6`, which is what `pr_ready.py` diffs against (§3.1).

## 3. Selection rules (quoted from main `41f822b6`, which is also live main)

### 3.1 Local `tools/pr_ready.py --quick`

- Changed files are `git diff --name-only $(git merge-base HEAD origin/main)` plus untracked files
  ([`tools/parity.py:1409-1418`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/parity.py#L1409-L1418), called from [`tools/pr_ready.py:577-594`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L577-L594)). `--base` defaults to
  `origin/main` ([`tools/pr_ready.py:579-579`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L579-L579)).
- **`--quick` only changes the Parity step.** The harnesses are listed instead of run
  ([`tools/pr_ready.py:398-407`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L398-L407)). rustfmt, clippy and **the cargo test step still run.**
- A path is workspace Rust when it is not `*.md` and starts with `crates/`, `xtask/`, `.cargo/`, `native-hooks.json` or
  `tools/native-hook-fragments/`, or is a root Rust file ([`tools/pr_ready.py:72-72`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L72-L72), [`tools/pr_ready.py:78-89`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L78-L89)).
- `touched_crates` ([`tools/pr_ready.py:97-118`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L97-L118)):
  - a root Rust file or `.cargo/` → `None` (whole workspace);
  - `native-hooks.json` / `tools/native-hook-fragments/` (`MOD_API_INPUTS`, [`tools/pr_ready.py:80-80`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L80-L80)) →
    **`horizon-client`, `horizon-client-render`, `horizon-mod-api-gen`** ([`tools/pr_ready.py:105-107`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L105-L107));
  - `crates/horizon-client/…` → `horizon-client` for gameplay files (`src/game/`, `game.rs`, `lib.rs`, `main.rs`,
    `entry.rs`, `Cargo.toml`, `build.rs`, …), otherwise `horizon-client-lib` ([`tools/pr_ready.py:108-114`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L108-L114));
  - anything else → the workspace member whose directory owns the path (longest match) ([`tools/pr_ready.py:115-117`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L115-L117)).
  - `plan()`: Rust changes with no recognised crate → whole workspace ([`tools/pr_ready.py:354-357`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L354-L357)).
- The test step is exactly `cargo test --locked <-p each touched crate | --workspace>[ --features horizon-mods/index] --tests`
  ([`tools/pr_ready.py:377-379`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L377-L379)). Only the **touched** crates get `-p`. `dependent_crates` widens only the
  clippy decision ([`tools/pr_ready.py:233-242`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L233-L242), [`tools/pr_ready.py:362-365`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L362-L365)), and clippy is
  `--workspace` anyway. `--tests` includes each selected package's lib unit-test target, so `-p horizon-client-render`
  runs `options_ui::tests`. Doc tests use `--doc` ([`tools/pr_ready.py:380-381`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L380-L381)), and the content-root step is
  `-p horizon-client-lib --lib mod_content::tests:: -- --ignored` ([`tools/pr_ready.py:346-346`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L346-L346)). Neither runs
  this test.
- **Building is not testing.** With `-p horizon-client` or `-p horizon-client-lib`, cargo compiles
  `horizon-client-render` as a dependency but does **not** run its unit tests. Only `-p horizon-client-render` (or
  `--workspace`) runs the render lib test binary that contains this test.
- `pr_ready.py` sets no audio variable and adds no timeout to cargo steps: it runs them in `bash -c` with the
  inherited environment ([`tools/pr_ready.py:423-437`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L423-L437)), and the only timeout in the file is the parity
  `--timeout 1500` ([`tools/pr_ready.py:410-410`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/pr_ready.py#L410-L410)). The 5-minute watchdog in the earlier local runs was
  external to `pr_ready.py`, and it watched only the `horizon_client_render` test binaries.

### 3.2 GitHub PR CI (`.github/workflows/ci.yml`, workflow `checks`)

- Triggers: `pull_request` `[opened, synchronize, reopened]`, push to main, dispatch ([`.github/workflows/ci.yml:10-15`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/ci.yml#L10-L15)).
- Job **`verify`**: runs on `["self-hosted","linux","horizon-checks"]` for PR numbers ending in 0 or 1, otherwise
  `ubuntu-latest`. The repository variable `SELF_HOSTED_CI` can override this (`all`/`light`/`off`; its value was not
  read). **Both pools are Linux.** `timeout-minutes: 45` ([`.github/workflows/ci.yml:36-42`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/ci.yml#L36-L42)).
- Path filter: `python3 tools/ci/rust_changes.py "$BASE_SHA" "$HEAD_SHA"` ([`.github/workflows/ci.yml:51-57`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/ci.yml#L51-L57)).
  `required` is true when any path in `git diff --name-only --no-renames base...head` is outside `docs/` and not
  `*.md` ([`tools/ci/rust_changes.py:22-23`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/ci/rust_changes.py#L22-L23), [`tools/ci/rust_changes.py:34-39`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/ci/rust_changes.py#L34-L39)). This is much broader
  than `pr_ready`'s crate selection: a `tools/` or `tests/` change alone turns it on.
- When required: `cargo +stable test --workspace --lib --bins --tests`, then the `mod_content::tests:: -- --ignored`
  run, then `--doc` ([`.github/workflows/ci.yml:109-117`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/ci.yml#L109-L117)). `--workspace` includes `horizon-client`, so
  render's `audio` feature is on (§5).
- The `verify` checkout has no `ref:` ([`.github/workflows/ci.yml:44-46`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/ci.yml#L44-L46)). For `pull_request` events,
  `actions/checkout` then defaults to GitHub's merge ref (`refs/pull/<n>/merge`), so CI tests the PR merged into the
  base of that moment. This is GitHub/`actions/checkout` default behaviour, not a rule in this repository. Because the
  test reached main on 2026-10-29 (§7) and every merge base here already contains it, CI's tree contains it either way.
- The other jobs in `ci.yml` don't run render's tests either:
  - `launcher` builds its own workspace ([`.github/workflows/ci.yml:121-161`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/ci.yml#L121-L161));
  - `parity-scope`/`parity` run harnesses ([`.github/workflows/ci.yml:260-294`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/ci.yml#L260-L294); see §3.3);
  - `windows-arm64` and `vanilla-parity` are `workflow_dispatch` only.
- Observed: the latest `verify` check-run on **every** one of the 114 primary heads (and on #10194/#10195) has
  conclusion `success` (`gh api repos/horizon-rust/horizon/commits/<head>/check-runs`). This shows that a Linux run
  finished on each head. It does not tell us anything about macOS.

### 3.3 Audit of every file in `.github/workflows/` (21 files, text only)

I checked each file's `on:` block and every `cargo`, `uses: ./…` and `run: python3/bash tools/…` line, then followed
the scripts that PR-triggered jobs call. Eight files have a `pull_request` or `pull_request_target` trigger:

| Workflow | PR trigger | Runner OS | How tests are run | Runs render's lib tests? |
|---|---|---|---|---|
| `ci.yml` (`checks`) | `pull_request` [opened, synchronize, reopened] | Linux | `verify`: direct `cargo +stable test --workspace --lib --bins --tests` (§3.2). `launcher`: `cargo test` in `launcher/`. `parity`: reusable `parity.yml` | **Yes, only in `verify`, on Linux** |
| `run-parity.yml` | `pull_request` [labeled, synchronize]; job only on the `run-parity` label ([L6-30](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/run-parity.yml#L6-L30)) | via `parity.yml` | reusable `parity.yml` | No (see below) |
| `mod-registry.yml` | `pull_request`, paths `crates/horizon-mods/**`, `crates/horizon-mod-registry/**`, the workflow | Linux (`horizon-checks` or `ubuntu-latest`) | direct `cargo +stable test --locked` in `crates/horizon-mod-registry` ([L28](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/mod-registry.yml#L28), [L40](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/mod-registry.yml#L40)) | No: separate `[workspace]` ([`Cargo.toml:11`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-mod-registry/Cargo.toml#L11)), no `horizon-client*` dependency |
| `telemetry-server.yml` | `pull_request`, paths `crates/horizon-telemetry/**`, `crates/horizon-telemetry-server/**`, the workflow | Linux | direct `cargo +stable test --locked` in `crates/horizon-telemetry-server` ([L28](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/telemetry-server.yml#L28), [L40](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/telemetry-server.yml#L40)) | No: separate `[workspace]` ([`Cargo.toml:11`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-telemetry-server/Cargo.toml#L11)), no `horizon-client*` dependency |
| `cargo-horizon-mod.yml` | `pull_request`, paths `crates/cargo-horizon-mod/**`, `tools/horizon_mod_binstall.py`, the workflow | Linux, Windows, **`macos-latest`** (`PLATFORMS`, [`tools/horizon_mod_binstall.py:19-25`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/tools/horizon_mod_binstall.py#L19-L25)) | **no `cargo test`**: `cargo build --release --locked -p cargo-horizon-mod` and starts the binary ([L49-61](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/cargo-horizon-mod.yml#L49-L61)) | No: no tests, and `cargo-horizon-mod`'s only path dependency is `horizon-mods` |
| `cancel-on-close.yml` | `pull_request_target` [closed] | Linux | `gh run cancel` loop, no cargo | No |
| `overlap-note.yml` | `pull_request_target` | Linux | `python3 tools/ci/overlap_note.py event`, no cargo | No |
| `pr-labels.yml` | `pull_request_target` | Linux | `python3 tools/ci/pr_labels.py event`, no cargo | No |

**`parity.yml`**, the reusable workflow behind `ci.yml`'s `parity` job and `run-parity.yml`, is the only PR path that
can reach a Mac. Its runner is `["self-hosted","horizon-game"]`, which includes the Mac mini ([L36-40](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/parity.yml#L36-L40);
`docs/ci-runners.md`). It runs no direct `cargo test`. Its steps are `runner-env.sh`, `vanilla-data.sh` (which only
does `cargo build --release --locked -p horizon-prepare`), `prepare_output.py`, `gen_version_consts.py`, and
`tools/parity.py --changed` ([L74-101](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/.github/workflows/parity.yml#L74-L101)).
`parity.py` itself builds probes with `cargo build` (`tools/parity.py:663`).

Some harness scripts do call `cargo test` themselves, but always on other crates' `--lib` targets with a test-name
filter:
- `-p horizon-client --lib` or `-p horizon-client-lib --lib` (breeze_hook, client_arrow, damage_catalog,
  dragon_contacts, dragon_render, fishing_rng, happy_ghast_leash, lan_auth, map_saved_data, mob_air, mob_fall_reset,
  mount_control, opened_loot, raider_celebration, shulker_space, villager_farming);
- `-p horizon-protocol --lib` (server_address).

**None passes `-p horizon-client-render` or `--workspace`.** `--lib` on `horizon-client` or `horizon-client-lib`
builds render as a dependency but never runs render's lib test binary, which is the only binary that contains this
test (§5). Other text hits were checked and excluded:
- `tools/test_launcher_macos_update.py` tests a temporary `Cargo.toml` it writes itself;
- `tests/test_pr_ready.py` only asserts command strings;
- `tools/gate.py` (`cargo test --workspace`) is run only by the local `tools/supervise.py`; no workflow calls it, and
  `tools/parity.py` only names it in comments and messages.

The remaining 13 files have no PR trigger:
- `claim-command`, `nightly-command`, `triage-command`: `issue_comment`;
- `mojang-republish`, `nightly`, `nightly-build`, `release`: schedule, `workflow_run` or dispatch;
- `launcher`, `native-hook-stamp`, `pack-bench`, `parity-main`: push or dispatch;
- `game-build`: `workflow_call` from `nightly-build`.

Two of these can run on macOS, but neither is triggered by a PR or runs `cargo test`:
- `game-build` builds on Mac runners plus a 1500-frame smoke run;
- `nightly` and `parity-main` run harnesses.

`launcher.yml`'s `cargo test` is the launcher workspace, triggered by push or schedule.

**Outcome:** no PR-triggered workflow other than `ci.yml`'s `verify` job (Linux) runs `horizon-client-render`'s lib
tests, and none runs them on macOS.

## 4. What the test does, and when it reaches a real audio backend

- The test asserts the Sound page row, then calls `crate::audio::output_devices()` and cycles `Opt::SoundDevice`
  forward and back `1 + devices.len()` times ([`crates/horizon-client-render/src/options_ui.rs:7602-7625`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-render/src/options_ui.rs#L7602-L7625)).
  Each `cycle` calls `crate::audio::output_devices()` again ([`crates/horizon-client-render/src/options_ui.rs:1091-1107`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-render/src/options_ui.rs#L1091-L1107)).
- `output_devices()` has two definitions ([`crates/horizon-client-render/src/audio.rs:918-941`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-render/src/audio.rs#L918-L941)):
  - `#[cfg(feature = "audio")]`: returns `Vec::new()` if `output_disabled(...)` is `Some`, otherwise
    `cpal::default_host().output_devices()` and each `device.name()`;
  - `#[cfg(not(feature = "audio"))]`: `Vec::new()`. This stub never touches `cpal`.
- `output_disabled` ([`crates/horizon-client-render/src/audio.rs:730-747`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-render/src/audio.rs#L730-L747)) returns `Some` when
  **`HORIZON_NO_AUDIO` is set (any value)**. It returns `None` when `HORIZON_AUDIO=1`. Otherwise it returns `Some`
  only when `WAYLAND_DISPLAY` has a `vscreen` path component.

### Correction to the brief

The brief said `HORIZON_NO_AUDIO` does not skip the enumeration path. **The source contradicts this.** At `41f822b6`,
and at the heads of #10194 and #10195, `output_devices()` (audio feature) returns before any `cpal` call when
`HORIZON_NO_AUDIO` is set in the **test process's** environment. That early return has been there since the function
was introduced (`git log -L` on the function: one commit, `48e731483`, #1035).

What the source does support:
- the test has no `#[ignore]` and no env or feature guard of its own;
- `pr_ready.py` neither sets nor removes `HORIZON_NO_AUDIO`, so a run without that variable in the shell takes the
  `cpal` path when `audio` is on.

So for the observed local hang, either `HORIZON_NO_AUDIO` was not in the cargo test process's environment, or the
parked thread did not come from this function. This note cannot tell which (§10).

## 5. Dependency and feature edges (from `Cargo.toml`, main `41f822b6`)

- Workspace `resolver = "2"`. `horizon-client`, `horizon-client-lib` and `horizon-client-render` are members
  ([`Cargo.toml:3-45`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/Cargo.toml#L3-L45)).
- `horizon-client-render`: `cpal` is optional ([`crates/horizon-client-render/Cargo.toml:37-37`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-render/Cargo.toml#L37-L37)). Its
  `[features]` define `audio = ["dep:cpal"]` and `test-support`, with **no `default`**
  ([`crates/horizon-client-render/Cargo.toml:72-76`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-render/Cargo.toml#L72-L76)).
- `horizon-client-lib` → `horizon-client-render` (normal dependency [`crates/horizon-client-lib/Cargo.toml:19-19`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-lib/Cargo.toml#L19-L19),
  dev dependency with `test-support` [`crates/horizon-client-lib/Cargo.toml:132-132`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-lib/Cargo.toml#L132-L132)).
  `default = ["audio", "controllers"]` and `audio = ["horizon-client-render/audio"]`
  ([`crates/horizon-client-lib/Cargo.toml:114-118`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-lib/Cargo.toml#L114-L118)). Its lib is `../horizon-client/src/client_lib.rs`
  ([`crates/horizon-client-lib/Cargo.toml:12-13`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-lib/Cargo.toml#L12-L13)), which **re-exports** `options_ui` and `audio` from render
  (`pub use horizon_client_render::{…}`, [`crates/horizon-client/src/client_lib.rs:11-26`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client/src/client_lib.rs#L11-L26)). The test is
  therefore compiled only into render's own lib test binary.
- `horizon-client` → `horizon-client-lib` (`default-features = false`, [`crates/horizon-client/Cargo.toml:15-15`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client/Cargo.toml#L15-L15))
  and → `horizon-client-render` ([`crates/horizon-client/Cargo.toml:19-19`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client/Cargo.toml#L19-L19); dev dependency with
  `test-support` [`crates/horizon-client/Cargo.toml:158-158`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client/Cargo.toml#L158-L158)). `default = ["audio", "controllers"]` and
  `audio = ["horizon-client-render/audio", "horizon-client-lib/audio"]` ([`crates/horizon-client/Cargo.toml:109-113`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client/Cargo.toml#L109-L113)).
- `horizon-client` and `horizon-client-render` both build-depend on `horizon-mod-api-gen`
  ([`crates/horizon-client/Cargo.toml:149-150`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client/Cargo.toml#L149-L150), [`crates/horizon-client-render/Cargo.toml:78-79`](https://github.com/horizon-rust/horizon/blob/41f822b659789780f58d32c63ddfb6887ea1b8bf/crates/horizon-client-render/Cargo.toml#L78-L79)).
  Render's `build.rs` generates the native mod API from the repo root, so hook-fragment JSON is an input to render's
  build. `horizon-mod-api-gen` has no workspace dependencies.
- No other workspace `Cargo.toml` depends on `horizon-client`, `horizon-client-lib` or `horizon-client-render`
  (`git grep 'horizon-client(-lib|-render)? *=' -- '*Cargo.toml'`).

**Consequence.** In one cargo invocation, render's `audio` feature is on if and only if `horizon-client` or
`horizon-client-lib` is among the selected packages with default features, or `--workspace` is used. In that case
`cargo test -p horizon-client-render …` builds render's test binary with `audio` → real `cpal`, which is CoreAudio on
macOS and ALSA on Linux. **`-p horizon-client-render` alone (or with crates that don't enable it) builds the stub**, so
the test still runs and passes trivially without touching the OS audio layer. The 5 "audio OFF" PRs are of that kind.
The same split showed up in every head checked: all 197 heads carry these same audio
edges (render has no default `audio`; client and client-lib default-enable it).

## 6. Why #10194 and #10195 tested horizon-client, -lib, -render and horizon-mod-api-gen

Neither PR modifies `audio.rs` or `options_ui.rs`. Their merge base is `41f822b6`, and their diffs are:

- #10194 (`6d9ed21235472fde03acc093e31804dc206eb560`): `crates/horizon-client-render/src/block_states.rs`, `crates/horizon-client-render/src/client_resources.rs`, `crates/horizon-client-render/src/lib.rs`, `crates/horizon-client/src/client_lib.rs`, `crates/horizon-client/src/game/native_block_sprites.rs`, `crates/horizon-client/src/mod_api.rs`, `docs/native-mods.md`, `tools/native-hook-fragments/block_appearance.json`
- #10195 (`cd86e8f45d778142c30e2198f0606c2f45789d34`): `crates/horizon-client-render/src/block_states.rs`, `crates/horizon-client-render/src/client_resources.rs`, `crates/horizon-client-render/src/lib.rs`, `crates/horizon-client/src/client_lib.rs`, `crates/horizon-client/src/game/native_block_sprites.rs`, `crates/horizon-client/src/game/native_client_services.rs`, `crates/horizon-client/src/mod_api.rs`, `docs/native-mods.md`, `tools/native-hook-fragments/block_appearance.json`, `tools/native-hook-fragments/client_entity_list.json`

Mapped by `touched_crates`:
- `tools/native-hook-fragments/*.json` is a `MOD_API_INPUTS` path, which adds `horizon-client`, `horizon-client-render`
  and `horizon-mod-api-gen`;
- `crates/horizon-client-render/src/{block_states,client_resources,lib}.rs` independently selects
  `horizon-client-render` through directory ownership;
- `crates/horizon-client/src/client_lib.rs` and `mod_api.rs` are non-gameplay, which adds `horizon-client-lib`;
- `crates/horizon-client/src/game/*.rs` is gameplay, which adds `horizon-client`;
- `docs/native-mods.md` is ignored.

Result: `cargo test --locked -p horizon-client -p horizon-client-lib -p horizon-client-render -p horizon-mod-api-gen --tests`.
`pr_ready` tests whole crates, not touched files, so render's entire lib test binary runs, including
`sound_device_button_in_sound_page_and_cycles`. Because `horizon-client`/`-lib` are in the same invocation, render is
compiled with `audio`. Both PRs are `unstable` today. Their latest `verify` (Linux) is `success`.

## 7. When the test arrived (dating)

- Introduced in commit `48e731483a9762a1c44b0bf51a50dfd5ed76d4fe` (2026-10-28 14:12 CDT, "options: choose the audio
  output device in Music & Sounds (#1035)"), in `crates/horizon-client/src/options_ui.rs`, together with
  `audio::output_devices()` and its `HORIZON_NO_AUDIO` early return.
- `horizon-client-render` was created by `a7efe597ea83e093fb816f22c93d6bd454fb5293` (2026-10-28 07:23 CDT).
- On main's first-parent history, render's `options_ui.rs` first contains the test at merge
  `909396071965b11de127fdcbf78e1953e8aeaa06` (2026-10-29 13:37 CDT, "Merge pull request #3355 from
  Sticky-Bug/feat/audio-output-device"). Its first parent `c7065027b` (2026-10-29 12:52 CDT) lacks it. This was found
  by bisecting first-parent main (it assumes the test was never removed and re-added).
- All times are git author dates converted to America/Chicago with
  `TZ=America/Chicago git log --date=format-local:'%Y-%m-%d %H:%M %Z'`.
- Every merge base (and every API base SHA) of the 197 clean+unstable heads descends
  from `909396071`. Every head's render `options_ui.rs` contains the test, none has it under
  `crates/horizon-client/src/options_ui.rs`, and the function text hashes the same as main's (`1427743e86ac`).
  **No examined branch gains the test by rebasing.** They all have it already.
- Caveat: `git log -S` over the file failed in this partial clone ("could not fetch … from promisor remote"). One
  early check printed a false "absent" because a blob fetch had failed silently. Every check reported here re-reads
  the blob and fails loudly instead.

## 8. Rebase effect (recorded SHA and live main are the same commit)

- Every head's `pr_ready.py` selection code (`ROOT_RUST`, `MOD_API_INPUTS`, `is_doc`, `workspace_rust`,
  `touched_crates`, and `plan()`'s fallback and test-command lines) is **textually identical** to main's (AST source
  segments hashed). So the head's own rules and main's rules agree; this was checked per head, not assumed.
- Nine primary heads are older and lack the newer workspace member `crates/horizon-mod-deps`:
  #9566, #10039, #10040, #10042, #10096, #10099, #10166, #10177, #10197. None of them changes that crate, so their selection is unchanged.
- The changed-path set used is the PR's own diff from its merge base. After a clean rebase onto main, `pr_ready` would
  diff against main and see the same paths. Re-evaluating with main's packages and manifests produced **0 differences
  in verdict or crate list** among all 197 heads.
- Limitations: conflict resolutions during a real rebase could change paths, and `mergeable_state` can change at any
  time.

## 9. Contrast: mergeable `unstable` / `behind`

- `behind`: **0** PRs.
- `unstable` (mergeable, non-draft, some non-required check failing or pending): **83** PRs, including
  #10194 and #10195. These were analysed the same way but are **not** part of the primary set. By local `--quick`
  verdict:
  - runs with `audio` ON: 12 (#10194, #10195, #10523, #11011, #11109, #11181, #11183, #11237, #11238, #11239, #11274, #11302);
  - runs with `audio` OFF: 11 (#10501, #11080, #11196, #11199, #11203, #11204, #11205, #11206, #11264, #11281, #11313);
  - render not selected: 56;
  - no workspace Rust: 4 (#10129, #11173, #11200, #11227).
- #10194 and #10195 are the known local-hang contrast. They are in the "audio ON" group with the 4-crate selection
  explained in §6.

## 10. Historical observation (as recorded, not reproduced here)

Recorded during the earlier local `pr_ready.py --quick` runs on #10194 and #10195 (macOS, Ian's Mac):
- pid 65599 was idle at 0% CPU for about 56 minutes;
- its stack was parked in CoreAudio `AudioComponentInstanceNew` → HAL plug-in → `mach_msg`, not in Horizon logic, so
  the process was alive and waiting on the OS audio layer;
- `system_profiler` still answered, so this was not a machine-wide CoreAudio deadlock;
- `pr_ready.py` exited 1 on both PRs because the test step failed when the external watchdog ended the hung run;
- that watchdog (5 minutes) watched only the `horizon_client_render` test binaries, not every test binary.

This note did not re-run anything. It did not re-check that the parked thread belonged to this test, or which `cpal`
call issued `AudioComponentInstanceNew`. The hang is not specific to #10194/#10195: any selection that runs render's
tests with `audio` on runs the same code.

## 11. Limitations

- "Would run" means the test is selected and compiled with real `cpal`. **It does not mean the test hangs.** No hang
  is reported for Linux CI (every head's `verify` succeeded), and **that is not proof that no hang is possible**.
  Nothing here claims a hang on Linux or on every Mac. The one hang observed was on one macOS machine.
- The local verdict assumes `pr_ready.py` is run as written: default `--base origin/main` with `origin/main` at
  `41f822b6`, a clean checkout of the PR head (uncommitted or untracked files would add paths), no
  `HORIZON_NO_AUDIO` (or a `vscreen` `WAYLAND_DISPLAY` without `HORIZON_AUDIO=1`) in the environment, and `--host local`.
  `--host macmini` runs cargo on the Mac mini instead.
- CI verdicts use the API's current `base.sha` for `rust_changes.py`. A past run used the base of its own event. The
  self-hosted vs `ubuntu-latest` choice depends on `SELF_HOSTED_CI`, which was not read. Both are Linux.
- `mergeable_state` is GitHub's point-in-time computation and can change without a push.
- The test was dated by a first-parent bisect, not `git log -S` (which failed in this partial clone), and assumes the
  test was not removed and re-added.
- The workflow audit (§3.3) is a text audit of `.github/workflows/*` at `41f822b6` and the scripts PR jobs call. It
  did not read repository variables or runner labels beyond `docs/ci-runners.md`, and it relies on the PR's merge
  commit using main's workflow files (a PR that edits a workflow could change its own CI; none of the 114 primary
  PRs changes a file under `.github/workflows/`).

## 12. Method (reproducible, read-only)

1. `gh api --paginate 'repos/horizon-rust/horizon/pulls?state=open&per_page=100'`, then `gh api repos/horizon-rust/horizon/pulls/<n>` for each PR.
2. `git fetch origin +refs/pull/<n>/head:refs/cpe/pr/<n>` (a local-only ref namespace), checked against `head.sha`.
3. For each head, from git objects only: merge base with `41f822b6`; `git diff --name-only <merge-base> <head>`; the
   workspace members and package names from each head's `Cargo.toml`; the audio feature edges; whether the test is
   present; and a hash of the head's `pr_ready.py` selection code. `touched_crates` was **re-typed from main** rather
   than executed from PR heads, so no PR code was run.
4. `gh api repos/horizon-rust/horizon/commits/<head>/check-runs` for `verify`/`launcher` conclusions.

## 13. Per-PR tables (primary set, every PR listed)

"Head SHA" and "API base SHA" are full SHAs. "Merge base (behind main)" is the merge base with `41f822b6` and how
many commits main has beyond it. "After rebase" says "same" when the verdict and crate list do not change (true for
all).

### 13.1 Runs the test with `audio` ON (25)

| PR | Title | Head SHA | API base SHA | Merge base (behind main) | Changed paths | Local `pr_ready --quick` at head | After rebase onto `41f822b6` | GitHub PR CI |
|---|---|---|---|---|---|---|---|---|
| [#10039](https://github.com/horizon-rust/horizon/pull/10039) | The selected item name and the action bar keep vanilla's height | `d0a71973e8aa0fceb555a61d2d4fece7065a9968` | `fb6121bce9fa724c2c28b1c4c0747cd70b93dc9d` | `fb6121bce9fa` (798) | 7 files: `crates/horizon-client`×4, `crates/horizon-client-render`×2, `tools/port_map`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10040](https://github.com/horizon-rust/horizon/pull/10040) | Draw shader API 0.2 packs in linear light behind HORIZON_HDR_FRAME=float (HDR step 2c) | `ffd9b8f475012ecc64d78ddb5810e8d927aba154` | `d545e9df609b92f548f2a530b536daf8e7477059` | `d545e9df609b` (786) | 28 files: `crates/horizon-client`×6, `crates/horizon-client-render`×11, `crates/horizon-shader-api`×8, `docs/api`×3 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-render, horizon-shader-api) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10168](https://github.com/horizon-rust/horizon/pull/10168) | Keep native shader selections and settings across updates | `1295602deb746c7dfed7f6aea19e730cbb34baf6` | `0873f24d2035eaa693a4b858d0f959ddbed12bdd` | `0873f24d2035` (34) | 10 files: `crates/horizon-client`×4, `crates/horizon-client-render`×4, `crates/horizon-shader-api`×1, `docs/api`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-render, horizon-shader-api) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10197](https://github.com/horizon-rust/horizon/pull/10197) | Native mods: client world snapshots a mod reads on its worker threads | `8f4888a2efb7a1eb9e6a194164afb30b4ea6ceac` | `6d99e14ff0c70ae60bc56b67829b526409fa068e` | `4f4a931b21d3` (639) | 5 files: `crates/horizon-client`×3, `docs`×1, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10245](https://github.com/horizon-rust/horizon/pull/10245) | Native hooks: AnvilUpdateEvent, a mod's say in an anvil's result | `2c92825807b9d09318d6849f653fcd6b388bf74b` | `64f691f9f75c11142a1ef3fcfe11efb43dd37b58` | `64f691f9f75c` (47) | 9 files: `crates/horizon-client`×7, `docs`×1, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10537](https://github.com/horizon-rust/horizon/pull/10537) | The dragon fight's world fog leaves out the sky, as vanilla does | `da067868483f3dc08b9da9c352b67ad9d84c658a` | `a02fdabe6f47d475e353eda108614416cd9d65f5` | `98af9399cd1c` (342) | `crates/horizon-client-render/src/fog.rs`<br>`crates/horizon-client-render/src/gfx.rs`<br>`crates/horizon-client/src/game.rs` | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-render) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10559](https://github.com/horizon-rust/horizon/pull/10559) | Reuse unchanged armor-trim preparation across frames | `786bd54096b991155bc2ef39bb7c365b888ffa59` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `f4dcf8a2fb4a` (26) | 6 files: `crates/horizon-client`×2, `crates/horizon-client-render`×2, `docs/performance`×2 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-render) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11010](https://github.com/horizon-rust/horizon/pull/11010) | Native hook catalog: tooltip_render_pre is RenderTooltipEvent.Pre; the Next hooks have all landed | `3eda4b1b1409392ff4f6c2ceada9a01f08762dc3` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `f4dcf8a2fb4a` (26) | `docs/native-hooks-neoforge.md`<br>`tools/native-hook-fragments/tooltip_render.json` | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-render, horizon-mod-api-gen) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11014](https://github.com/horizon-rust/horizon/pull/11014) | Shader packs: at_velocity for entities | `d8160a3ded79763f57e32f2390202565947a0020` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `f4dcf8a2fb4a` (26) | 12 files: `crates/horizon-client`×1, `crates/horizon-client-render`×10, `crates/horizon-shaders`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-render, horizon-shaders) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11019](https://github.com/horizon-rust/horizon/pull/11019) | Native mods: server hooks and helpers for plugin ports (console, connection, permissions, commands, login, players, inventory, movement, vehicles, chunks) | `964454e1f8178615996ac1414c113387d34e0038` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `f4dcf8a2fb4a` (26) | 105 files: `crates/horizon-client`×41, `crates/horizon-client-render`×1, `crates/horizon-mod-api-gen`×1, `crates/horizon-protocol`×1, `crates/horizon-server-files`×1, `crates/horizon-world`×1, `docs`×1, `tools/native-hook-fragments`×58 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen, horizon-mod-api-gen-tests, horizon-protocol, horizon-server-files, horizon-world) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11028](https://github.com/horizon-rust/horizon/pull/11028) | Native mods: more server hooks and helpers for plugin ports (feedback, packets, visuals, NPCs, damage, menus, worldgen, diagnostics, deps) | `962d7c6a223f3cd2f923ff2ff8428d20ca9ffaf6` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `f4dcf8a2fb4a` (26) | 98 files: `Cargo.lock`×1, `crates/horizon-client`×44, `crates/horizon-client-render`×1, `crates/horizon-items`×1, `crates/horizon-mod-api-gen`×1, `crates/horizon-mod-deps`×1, `crates/horizon-world`×7, `docs`×1, `tools/native-hook-fragments`×41 | **RUNS test, `audio` ON** (tests: workspace) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11051](https://github.com/horizon-rust/horizon/pull/11051) | Native mods: expose dropped-item block contact | `c4c0e6fc6b62713db22d22c094cc69c9203f1f91` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `f4dcf8a2fb4a` (26) | 8 files: `crates/horizon-client`×7, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11062](https://github.com/horizon-rust/horizon/pull/11062) | Add local particle spawning for native client mods | `f1ef5cb53a96cd7b32607212c3755c8b547b1c5d` | `28a389b7a37373598391d8e18d87846bc4218e41` | `f4dcf8a2fb4a` (26) | 5 files: `crates/horizon-client`×3, `tools`×1, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-render, horizon-mod-api-gen) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11070](https://github.com/horizon-rust/horizon/pull/11070) | Native mods: select the stack returned by Pick Block | `b55e8616c1bc6804fb449356c9f1ed989239ec2b` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `a1ff75a4a525` (21) | 7 files: `crates/horizon-client`×3, `crates/horizon-mod-api-gen`×2, `docs`×1, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen, horizon-mod-api-gen-tests) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11076](https://github.com/horizon-rust/horizon/pull/11076) | Add native client sound parameter interception | `58a5690ec61109c875cd9761813b412ea0c6416d` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `f4dcf8a2fb4a` (26) | 12 files: `crates/horizon-client`×8, `crates/horizon-client-render`×2, `crates/horizon-mod-api-gen`×1, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen, horizon-mod-api-gen-tests) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11090](https://github.com/horizon-rust/horizon/pull/11090) | Native mods: let existing block fluid helpers reach native-owned tanks | `c968fb0d7dce9ab7e31ba1c522ad74111ac982f2` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | 5 files: `crates/horizon-client`×3, `crates/horizon-mod-api-gen`×1, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen, horizon-mod-api-gen-tests) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11092](https://github.com/horizon-rust/horizon/pull/11092) | Skip falling-block mesh preparation behind the camera | `52766de1c4bea12b586061cf7178e245917120f8` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `a1ff75a4a525` (21) | 6 files: `crates/horizon-client`×1, `crates/horizon-client-render`×3, `docs/performance`×2 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-render) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11098](https://github.com/horizon-rust/horizon/pull/11098) | Native mods: create block entities from the actual state | `7af3e1ec44c451f7de3b991e838c422a0e25790d` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | 19 files: `crates/horizon-client`×11, `crates/horizon-client-render`×1, `crates/horizon-mod-api-gen`×2, `crates/horizon-world`×3, `tests`×1, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen, horizon-mod-api-gen-tests, horizon-world) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11099](https://github.com/horizon-rust/horizon/pull/11099) | Native mods: item use after a mob passes interaction | `ec74bd066b146a56b8d4305b5ed282f4db9677f9` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | 8 files: `crates/horizon-client`×5, `docs`×1, `docs/validation`×1, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11111](https://github.com/horizon-rust/horizon/pull/11111) | Expose focused text fields before character insertion | `87386f72d91a9b1e074df62d60eadc911b13fb03` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `f4dcf8a2fb4a` (26) | 16 files: `crates/horizon-client`×13, `crates/horizon-client-render`×1, `crates/horizon-mod-api-gen`×1, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen, horizon-mod-api-gen-tests) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11113](https://github.com/horizon-rust/horizon/pull/11113) | Native mods: helpers for server-side player bots (one movement tick, block collision boxes) | `cdd4ffe95d88b74709d203c18d41611a38c91f9e` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `a1ff75a4a525` (21) | 5 files: `crates/horizon-client`×3, `docs`×1, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11115](https://github.com/horizon-rust/horizon/pull/11115) | Native API: consume transient block entity client fields | `3ba55cc9fa3e37368bcf3dcdd82546e4f9a0b3d8` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | 6 files: `crates/horizon-client`×4, `crates/horizon-client-render`×1, `docs`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11116](https://github.com/horizon-rust/horizon/pull/11116) | Native content: resolve block geometry by entity context | `be3fbc2d2b927e68efaa1b94fe0feaf28951163f` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | 19 files: `crates/horizon-client`×8, `crates/horizon-client-render`×1, `crates/horizon-core`×1, `crates/horizon-mods`×1, `crates/horizon-world`×6, `docs`×2 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-core, horizon-mods, horizon-world) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11117](https://github.com/horizon-rust/horizon/pull/11117) | Native rendering: draw a still-fluid volume in a block entity | `d705f96e06a9256e528727b265a4d049030a988d` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | 17 files: `THIRD_PARTY_NOTICES.md`×1, `crates/horizon-client`×6, `crates/horizon-client-render`×7, `crates/horizon-mod-api-gen`×1, `tools/native-hook-fragments`×2 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen, horizon-mod-api-gen-tests) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11118](https://github.com/horizon-rust/horizon/pull/11118) | Native rendering: draw terrain models with an explicit material | `a3b1e9dd0887cf7c365f3280cb6ed8fb26dceafd` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | 25 files: `THIRD_PARTY_NOTICES.md`×1, `crates/horizon-assets`×2, `crates/horizon-client`×7, `crates/horizon-client-render`×11, `crates/horizon-mod-api-gen`×1, `tools/native-hook-fragments`×3 | **RUNS test, `audio` ON** (tests: horizon-assets, horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen, horizon-mod-api-gen-tests) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |

### 13.2 Runs the test with `audio` OFF, stub `output_devices()` (5)

| PR | Title | Head SHA | API base SHA | Merge base (behind main) | Changed paths | Local `pr_ready --quick` at head | After rebase onto `41f822b6` | GitHub PR CI |
|---|---|---|---|---|---|---|---|---|
| [#10531](https://github.com/horizon-rust/horizon/pull/10531) | Shader packs: a pack's pipelines compile together on several threads | `b02930a2f2b07dbaafc57e36e44752f322e86788` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | 4 files: `crates/horizon-client-render`×1, `crates/horizon-shaders`×3 | runs test, `audio` OFF → stub `output_devices()` (tests: horizon-client-render, horizon-shaders) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10556](https://github.com/horizon-rust/horizon/pull/10556) | Light a section from its palette's light info, not its state ids | `ddb365e6b470b3e321272158db7646235c29ba99` | `64f691f9f75c11142a1ef3fcfe11efb43dd37b58` | `64f691f9f75c` (47) | `crates/horizon-client-render/src/light.rs`<br>`crates/horizon-core/src/paletted.rs` | runs test, `audio` OFF → stub `output_devices()` (tests: horizon-client-render, horizon-core) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11054](https://github.com/horizon-rust/horizon/pull/11054) | gfx: skip broken Haswell Vulkan driver (hasvk) and fall back to OpenGL | `38d23bc13cd61b05021f145565980b02a69dbee7` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `a1ff75a4a525` (21) | `crates/horizon-client-render/src/gfx.rs` | runs test, `audio` OFF → stub `output_devices()` (tests: horizon-client-render) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11144](https://github.com/horizon-rust/horizon/pull/11144) | audio: support HORIZON_AUDIO_BUFFER override for Linux buffer underruns | `89dc5664ded40d432efc4ac69c3e9946da21e831` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | `crates/horizon-client-render/src/audio.rs`<br>`docs/native-client.md` | runs test, `audio` OFF → stub `output_devices()` (tests: horizon-client-render) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11162](https://github.com/horizon-rust/horizon/pull/11162) | Preserve literal quotation characters in modern sign messages | `b12708976009514ba384178a576cb4d6ba283bd9` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | 4 files: `crates/horizon-client-render`×1, `docs/parity`×1, `tools`×1, `tools/port_map`×1 | runs test, `audio` OFF → stub `output_devices()` (tests: horizon-client-render) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |

### 13.3 Does not run the test: render not selected (67)

| PR | Title | Head SHA | API base SHA | Merge base (behind main) | Changed paths | Local `pr_ready --quick` at head | After rebase onto `41f822b6` | GitHub PR CI |
|---|---|---|---|---|---|---|---|---|
| [#9566](https://github.com/horizon-rust/horizon/pull/9566) | Server panel: the launcher's Glass look, a page for each part, in-page dialogs | `82f601072bf06033f35b9224939c99431b230be3` | `83ad384f48d0742746d518737511882680aab5b7` | `83ad384f48d0` (1814) | 4 files: `crates/horizon-server`×4 | does not run (render not built; tests: horizon-server) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#9952](https://github.com/horizon-rust/horizon/pull/9952) | Iron golems toss by knockback resistance and keep their anger like vanilla | `d1ce5ddfd93cb0963c46c6099c1d4ae9f6e23a9c` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `a1ff75a4a525` (21) | 10 files: `crates/horizon-client`×9, `tools`×1 | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#9953](https://github.com/horizon-rust/horizon/pull/9953) | Port pack: 'events', a mod's NeoForge listeners against the hooks | `a6661aab7e92aa060ca268e13928cae1b7a8b373` | `64f691f9f75c11142a1ef3fcfe11efb43dd37b58` | `64f691f9f75c` (47) | 7 files: `crates/horizon-port-pack`×5, `docs`×2 | does not run (render not built; tests: horizon-port-pack) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10086](https://github.com/horizon-rust/horizon/pull/10086) | Multiplayer: sneaking for a use is the input of the last tick, as vanilla's isShiftKeyDown | `f15541c0feadc08ada12332ad1faf1656271f661` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | `crates/horizon-client/src/game/remote.rs` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10096](https://github.com/horizon-rust/horizon/pull/10096) | RCON: one password guess per connection | `3b71256fcdce8f2ec633e18d4a0359b8cc0bcc49` | `fb6121bce9fa724c2c28b1c4c0747cd70b93dc9d` | `fb6121bce9fa` (798) | `crates/horizon-server/src/rcon.rs` | does not run (render not built; tests: horizon-server) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10099](https://github.com/horizon-rust/horizon/pull/10099) | RSA: constant-time private-key operations | `e0b19571ddb674648b5c8a8b0415cdeacc0f9a6b` | `fb6121bce9fa724c2c28b1c4c0747cd70b93dc9d` | `fb6121bce9fa` (798) | `crates/horizon-protocol/src/rsa.rs` | does not run (render not built; tests: horizon-protocol) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10133](https://github.com/horizon-rust/horizon/pull/10133) | A mounted mob reaches from its vehicle's width, and a rider is hit from its seat up | `09160dc1e2fa96ce679070cd8ca248a2550a80f0` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `a1ff75a4a525` (21) | 14 files: `crates/horizon-client`×14 | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10177](https://github.com/horizon-rust/horizon/pull/10177) | fix(client): prevent taskbar overlap and window shift in borderless fullscreen | `313f65e049adacdc013863b80fe0ba0630bf5036` | `5b4823c19822088fed8b037c25d9b1ab4222bbb5` | `b9913a1902d6` (849) | `crates/horizon-client-input/src/window.rs`<br>`crates/horizon-client/src/game.rs`<br>`crates/horizon-client/src/game/options.rs` | does not run (render built as a dependency only; tests: horizon-client, horizon-client-input) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10208](https://github.com/horizon-rust/horizon/pull/10208) | Breaking a painting, a cushion or a boat makes a vibration | `35e7942094582969437d4864d58cf0ba7a711697` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | 10 files: `crates/horizon-client`×8, `tools`×1, `tools/port_map`×1 | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10240](https://github.com/horizon-rust/horizon/pull/10240) | Dropped items: cull at the box the frame draws, not the tick's | `f217a345931d5c81f9d23d005d0b33a9cb0fdc69` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | `crates/horizon-client/src/game/item_draw.rs`<br>`tools/port_map/client-004.tsv` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10242](https://github.com/horizon-rust/horizon/pull/10242) | Save default icon flags in custom potion effects | `7c32d1c4d6ec11e72c0e9392efd37b55d186286a` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | 5 files: `crates/horizon-items`×1, `tools`×4 | does not run (render not built; tests: horizon-items) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10365](https://github.com/horizon-rust/horizon/pull/10365) | Entity saves: a warden's anger keeps its suspects by UUID | `58f31cdf25d8b956c0e651c669260556ea51f130` | `64f691f9f75c11142a1ef3fcfe11efb43dd37b58` | `64f691f9f75c` (47) | 12 files: `crates/horizon-client`×10, `tools/port_map`×2 | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10373](https://github.com/horizon-rust/horizon/pull/10373) | multiplayer: narrate incompatible server in server list | `4e6b8875351280daad46c31aca9de2cb12a614dd` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | `crates/horizon-client/src/multiplayer.rs`<br>`docs/narrator.md` | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10527](https://github.com/horizon-rust/horizon/pull/10527) | dedicated: transfer-status, the counts a proxy reads before it sends players | `db1cde61a89fb02b8c76e07a5578e280c7099e5b` | `0d21d5520ec26c49b774bf1533e53ea8af81278d` | `98af9399cd1c` (342) | 5 files: `crates/horizon-client`×3, `docs`×1, `tools`×1 | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10542](https://github.com/horizon-rust/horizon/pull/10542) | Omit the default amplifier from saved player effects | `0efe1c2c13459829f03763fc6adc76c38ce7ae82` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | `crates/horizon-client/src/save.rs`<br>`tools/test_player_effect_amplifier_parity.py` | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10546](https://github.com/horizon-rust/horizon/pull/10546) | Fix HappyGhast ambient sound position | `5b12729b5fc9e9ae25e1b6132effe12834028189` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | `crates/horizon-client/src/mobs/ai.rs` | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10551](https://github.com/horizon-rust/horizon/pull/10551) | Peaceful drops a mob's player target and a golem's anger | `ed739050aea81aff74256a96493266bfb044ff4f` | `64f691f9f75c11142a1ef3fcfe11efb43dd37b58` | `64f691f9f75c` (47) | `crates/horizon-client/src/mobs/ai.rs` | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10552](https://github.com/horizon-rust/horizon/pull/10552) | Write surface rule runs of one block at once | `16cdc3afd9f8514e0b1cc93a0f73dc7664621205` | `64f691f9f75c11142a1ef3fcfe11efb43dd37b58` | `64f691f9f75c` (47) | 5 files: `crates/horizon-world`×5 | does not run (render not built; tests: horizon-world) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10553](https://github.com/horizon-rust/horizon/pull/10553) | Write chunk-section blocks with one position lookup and walked words | `82ac14c4ad9f038beb95af11d06319757c0abfee` | `64f691f9f75c11142a1ef3fcfe11efb43dd37b58` | `64f691f9f75c` (47) | `crates/horizon-core/src/storage.rs`<br>`crates/horizon-world/src/chunk.rs` | does not run (render not built; tests: horizon-core, horizon-world) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10554](https://github.com/horizon-rust/horizon/pull/10554) | Parse entity-models.json once at startup | `1caa6c8931823e7e4fa8d94e5d6672c84e1d806f` | `64f691f9f75c11142a1ef3fcfe11efb43dd37b58` | `64f691f9f75c` (47) | 4 files: `crates/horizon-client`×4 | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10564](https://github.com/horizon-rust/horizon/pull/10564) | Arrows and ender pearls now redirect ghast fireballs instead of passing through them | `bc6b668820dc7c7d89b011b8510bf988e7102356` | `0873f24d2035eaa693a4b858d0f959ddbed12bdd` | `d3f799f5d51d` (68) | `crates/horizon-client/src/game/creatures.rs`<br>`crates/horizon-client/src/game/native_projectiles.rs` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10565](https://github.com/horizon-rust/horizon/pull/10565) | Match entity NBT boolean loads to numeric byte conversion | `a2090d0b0ff4f7604f3de088239f698175ca9258` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | `crates/horizon-client/src/entity_nbt.rs`<br>`tools/test_entity_nbt_boolean_parity.py` | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10570](https://github.com/horizon-rust/horizon/pull/10570) | NBT: read and write one-byte strings whole | `3d47396e9b375136c0716ddd6aa7750f23f5a070` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `0d21d5520ec2` (338) | `crates/horizon-world/src/nbt.rs` | does not run (render not built; tests: horizon-world) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10571](https://github.com/horizon-rust/horizon/pull/10571) | Text displays: keep their glyphs, and skip the ones whose text is off screen | `079d82f968ffea284e9015f083f9a14f57b31a00` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `208f1c010fbf` (289) | `crates/horizon-client/src/game/text_displays.rs`<br>`crates/horizon-client/src/text_display.rs` | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10572](https://github.com/horizon-rust/horizon/pull/10572) | Content API: --launch-api says what content a build loads, and docs/api/content.md (#10562) | `10ec872cc1c27f03bb5825876a559a248dffaf24` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `64f691f9f75c` (47) | 5 files: `crates/horizon-client`×1, `docs/api`×4 | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10703](https://github.com/horizon-rust/horizon/pull/10703) | Region chunks: inflate zlib straight into the chunk's buffer | `0df8f65edac7cd316782d47ca77623455f5b5eff` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `0873f24d2035` (34) | `crates/horizon-world/src/compression.rs` | does not run (render not built; tests: horizon-world) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10837](https://github.com/horizon-rust/horizon/pull/10837) | Omit Mob-only flags from modeled boat, stand and Item saves | `3122ae98c09f9db642b3fba2bfa78d1acdd187a6` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `0b2f2fda7392` (29) | `crates/horizon-client/src/entity_nbt.rs`<br>`docs/parity/non-mob-saved-flags.md`<br>`tools/test_non_mob_saved_flags_parity.py` | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10908](https://github.com/horizon-rust/horizon/pull/10908) | A thrown trident hurts other players on a served world | `6285cca37c9129ea1900f56a1eb4aba79f01c48b` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `d3f799f5d51d` (68) | `crates/horizon-client/src/game/trident.rs` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10967](https://github.com/horizon-rust/horizon/pull/10967) | A mob's damaging fall plays the fall sound of the block it lands on | `d49f1a0444327e1e717c56579496131916bd7c50` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `f4dcf8a2fb4a` (26) | `crates/horizon-client/src/mobs/living.rs` | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10996](https://github.com/horizon-rust/horizon/pull/10996) | A sleeping villager, fox or player isn't pushed by the mobs that walk into it | `ad6e2683836e1cab10593f084c70c95c620d24a3` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `f4dcf8a2fb4a` (26) | `crates/horizon-client/src/mobs.rs`<br>`crates/horizon-client/src/mobs/ai.rs` | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11007](https://github.com/horizon-rust/horizon/pull/11007) | A use that swings the hand resets the attack charge | `1de40d7cd38abb2a166133b4b75fd9d4622f9f12` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `d3f799f5d51d` (68) | `crates/horizon-client/src/game/lan_actions.rs` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11009](https://github.com/horizon-rust/horizon/pull/11009) | A villager's held-up trade item never drops when it dies | `225bcde8b98b1e2eeac5b24a112540f7a3eb9f47` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `f4dcf8a2fb4a` (26) | `crates/horizon-client/src/mobs/villager_brain.rs`<br>`crates/horizon-client/src/mobs/villager_trades_tests.rs` | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11016](https://github.com/horizon-rust/horizon/pull/11016) | Keep one world border per level | `c545bd4cb198130a18100be84cf2b770985755a7` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `f4dcf8a2fb4a` (26) | 5 files: `crates/horizon-client`×5 | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11017](https://github.com/horizon-rust/horizon/pull/11017) | Award crafted-item statistics for merchant result takes | `39c12c2542c51dfa8a752862bc24fe6d2339873a` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `f4dcf8a2fb4a` (26) | 9 files: `crates/horizon-client`×4, `mods`×1, `tools`×3, `tools/port_map`×1 | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11022](https://github.com/horizon-rust/horizon/pull/11022) | Cauldrons, beehives, snow golems, farming villagers and lead knots make the vibrations vanilla's do | `fac11b3d6aa59549e3730ef9d2e2b4579196693b` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `f4dcf8a2fb4a` (26) | 10 files: `crates/horizon-client`×10 | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11023](https://github.com/horizon-rust/horizon/pull/11023) | Casting and reeling a fishing rod and throwing an eye of ender make the vibrations vanilla's do | `e3ca63226c342a794c12d8ce4355ca57b55b24ec` | `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd` | `f4dcf8a2fb4a` (26) | 5 files: `crates/horizon-client`×5 | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11030](https://github.com/horizon-rust/horizon/pull/11030) | Merge remote-tracking branch 'origin/main' into fix/illager-target-memory | `01954d87c5b64dab03849466fcedfed40ee39317` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | `crates/horizon-client/src/mobs/goals.rs`<br>`crates/horizon-client/src/mobs/illager.rs`<br>`crates/horizon-client/src/mobs/illusioner.rs` | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11032](https://github.com/horizon-rust/horizon/pull/11032) | Merge remote-tracking branch 'origin/main' into fix/piglin-admired-drop | `cd7ce3024a54653689c6dfce5a5eb2830cc989b7` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `f4dcf8a2fb4a` (26) | `crates/horizon-client/src/game/piglins.rs`<br>`crates/horizon-client/src/mobs/piglin.rs` | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11033](https://github.com/horizon-rust/horizon/pull/11033) | Death protection: use the death_protection item and its effects, for players and mobs | `8a78ad33159aa92865cfe435965072e78f10dc6f` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `f4dcf8a2fb4a` (26) | 4 files: `crates/horizon-client`×3, `crates/horizon-items`×1 | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib, horizon-items) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11036](https://github.com/horizon-rust/horizon/pull/11036) | Interpolate whole-cell volumes four columns at a time; skip air runs at once | `aba1b1eca4a91548792a050304929fb5109f37e0` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `f4dcf8a2fb4a` (26) | `crates/horizon-core/src/density/interpolation.rs`<br>`crates/horizon-world/src/worldgen/terrain.rs` | does not run (render not built; tests: horizon-core, horizon-world) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11043](https://github.com/horizon-rust/horizon/pull/11043) | Validate resource identifiers when reading packets | `5fd9d40d4cb772ef07e3fda9795dabeeba7ba9dd` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `f4dcf8a2fb4a` (26) | 5 files: `crates/horizon-protocol`×3, `mods`×1, `tools`×1 | does not run (render not built; tests: horizon-protocol) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11044](https://github.com/horizon-rust/horizon/pull/11044) | Pearls on a server's client hover in bubble columns as vanilla's do | `09ac779466cfc1219584796500c7921d5892c45e` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `f4dcf8a2fb4a` (26) | `crates/horizon-client/src/game/creatures.rs`<br>`crates/horizon-client/src/game/remote_projectiles.rs`<br>`crates/horizon-client/src/projectiles.rs` | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11046](https://github.com/horizon-rust/horizon/pull/11046) | Honor remapped function keys from XKB | `d2d046b1be7e11c00b22b4f0642f35908cdeae35` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `f4dcf8a2fb4a` (26) | `crates/horizon-client/src/game.rs`<br>`crates/horizon-client/src/game/keymap_tests.rs` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11049](https://github.com/horizon-rust/horizon/pull/11049) | Upgrade region files in parallel in Optimize World | `d3d24aca55f3b00973e7fab426d1b08f225eaefe` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | `crates/horizon-world/src/datafix/world_upgrader.rs`<br>`crates/horizon-world/src/datafix/world_upgrader_tests.rs` | does not run (render not built; tests: horizon-world) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11056](https://github.com/horizon-rust/horizon/pull/11056) | Save default Interaction dimensions and response | `40b1fc9100f70e358e85d51da9965cee337a33ba` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `a1ff75a4a525` (21) | 4 files: `crates/horizon-client`×1, `docs/parity`×1, `tools`×1, `tools/port_map`×1 | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11057](https://github.com/horizon-rust/horizon/pull/11057) | Save common Entity fields for registered Markers | `62028656e07a52cb4eb12709f858d256d78bdd35` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `f4dcf8a2fb4a` (26) | `crates/horizon-client/src/entity_nbt.rs`<br>`crates/horizon-client/src/game/persist.rs` | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11065](https://github.com/horizon-rust/horizon/pull/11065) | Match Java numeric server properties | `82dc682cbcf8b3accd197d35eedab7ad8d4f936c` | `28a389b7a37373598391d8e18d87846bc4218e41` | `f4dcf8a2fb4a` (26) | 5 files: `crates/horizon-core`×1, `crates/horizon-server-files`×2, `mods`×1, `tools`×1 | does not run (render not built; tests: horizon-core, horizon-server-files) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11067](https://github.com/horizon-rust/horizon/pull/11067) | Match NBT path quoted escapes and index tokens | `db25d1d0927cb2b1ee5a292c796755dbfba43241` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `f4dcf8a2fb4a` (26) | 4 files: `crates/horizon-client-core`×2, `mods`×1, `tools`×1 | does not run (render not built; tests: horizon-client-core) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11068](https://github.com/horizon-rust/horizon/pull/11068) | Content mods: choose light emission per block state | `d2cb5a272bfc85b2c6dd00ba6f04fa342a220d72` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `a1ff75a4a525` (21) | 5 files: `crates/horizon-client`×2, `crates/horizon-mods`×1, `docs`×2 | does not run (render built as a dependency only; tests: horizon-client-lib, horizon-mods) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11071](https://github.com/horizon-rust/horizon/pull/11071) | Omit stale default power from saved TNT | `d2b61ade9178d1c55703ebb889e8be71b962d8da` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `a1ff75a4a525` (21) | `crates/horizon-client/src/entity_nbt.rs`<br>`docs/parity/tnt-saved-power.md`<br>`tools/test_tnt_saved_power_parity.py` | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11083](https://github.com/horizon-rust/horizon/pull/11083) | Load sound assets from combined native mods | `58b69f8f71be0348ff46181de0dbc3c9c8b11492` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `f4dcf8a2fb4a` (26) | 103 files: `crates/cargo-horizon-mod`×2, `crates/horizon-client`×4, `crates/horizon-mods`×2, `docs`×1, `docs/api`×1, `launcher`×93 | does not run (render built as a dependency only; tests: cargo-horizon-mod, horizon-client, horizon-mods) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11100](https://github.com/horizon-rust/horizon/pull/11100) | Native mods: choose the break slot for held-item durability | `3de12c0c7c1c17da78c522280a9ecafe06c2c7e0` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | `crates/horizon-client/src/game.rs`<br>`crates/horizon-client/src/game/native_hooks.rs` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11107](https://github.com/horizon-rust/horizon/pull/11107) | Older clients: 26.2's packet layouts for 26.2 clients (#10404, step 2) | `8dad7f0ac026cd6aa9bbb169f2d6e515b84746d4` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | 9 files: `crates/horizon-client`×1, `crates/horizon-protocol`×8 | does not run (render built as a dependency only; tests: horizon-client-lib, horizon-protocol) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11108](https://github.com/horizon-rust/horizon/pull/11108) | Remove stale Animal LoveCause when no reference is modeled | `6e302fab4876b83c1195fb3a4784aeed324c7302` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | 4 files: `crates/horizon-client`×1, `docs/parity`×1, `tools`×1, `tools/port_map`×1 | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11114](https://github.com/horizon-rust/horizon/pull/11114) | Native content: place a block through an item alias | `1bb6912a4b9d41108302e8295640f95e8aad0488` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | 7 files: `crates/horizon-client`×3, `crates/horizon-items`×1, `crates/horizon-mods`×1, `docs`×2 | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib, horizon-items, horizon-mods) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11122](https://github.com/horizon-rust/horizon/pull/11122) | Preserve live mod block entities when loading command data | `90b9df2899d7b7be9ba0562fc959ecf24ff5e5b0` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | `crates/horizon-client/src/game/commands/data.rs`<br>`crates/horizon-client/src/game/commands/data/mod_block_entity_tests.rs`<br>`docs/validation/mod-data-in-place.md` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11127](https://github.com/horizon-rust/horizon/pull/11127) | Hide discarded mobs from immediate lookup and passenger saves | `e575a22cb219fc85ab2a3e34400c0aba3c39cb04` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | 8 files: `crates/horizon-client`×5, `docs/validation`×1, `mods`×1, `tools`×1 | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11129](https://github.com/horizon-rust/horizon/pull/11129) | Avoid inventory notifications when provisioning empty native slots | `a5fa6f0d091d1af327d2247f7dac77c712132ffc` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | `crates/horizon-client/src/game/native_hooks.rs` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11133](https://github.com/horizon-rust/horizon/pull/11133) | An enderman hurt by the game layer teleports at the hurt, as vanilla's does | `0b0a58af3e2bfc3c2391cfd726c4d44be3a84726` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `28a389b7a373` (16) | 8 files: `crates/horizon-client`×7, `tools`×1 | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11135](https://github.com/horizon-rust/horizon/pull/11135) | Adding support for installing from PrismLauncher-portable | `266040ff8e4ecb09af89ed8e63cfc886e6fbb59c` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | 5 files: `crates/horizon-prepare`×5 | does not run (render not built; tests: horizon-prepare) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11136](https://github.com/horizon-rust/horizon/pull/11136) | Split slimes, magma cubes and sulfur cubes when the body is removed | `0f992292b7f8ad5409589568ed3ab70216b2dfb5` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | `crates/horizon-client/src/game/creatures.rs` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11139](https://github.com/horizon-rust/horizon/pull/11139) | Preserve Pillager and WanderingTrader saved inventories | `85b299df9dc527d79e0b997f5f1d970299e2f83b` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | 6 files: `crates/horizon-client`×2, `docs/parity`×1, `tools`×1, `tools/port_map`×2 | does not run (render built as a dependency only; tests: horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11152](https://github.com/horizon-rust/horizon/pull/11152) | Show the dismount hint when the player starts riding | `7a46ba1f27193126a095c75b30b710d3175f4a64` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | `crates/horizon-client/src/game/remote_vehicles.rs` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11153](https://github.com/horizon-rust/horizon/pull/11153) | Seat other players on a cushion they sit on | `f3d842d846c8b27fecbdeaf36021d62e3a89faf4` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | `crates/horizon-client/src/game/cushions.rs`<br>`crates/horizon-client/src/game/passengers.rs` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11155](https://github.com/horizon-rust/horizon/pull/11155) | Fix live mob custom-name component predicates | `253a6948712a71f7efcaf6b3dabf47b5ef9c0236` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | 6 files: `crates/horizon-client`×2, `tools`×3, `tools/port_map`×1 | does not run (render built as a dependency only; tests: horizon-client, horizon-client-lib) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11157](https://github.com/horizon-rust/horizon/pull/11157) | Honor fork state for terminal execute condition errors | `4d3f9944eb7ee3ae8c736a639b760b6a43abd345` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | 4 files: `crates/horizon-client`×1, `docs/parity`×1, `tools`×1, `tools/port_map`×1 | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11159](https://github.com/horizon-rust/horizon/pull/11159) | Minecarts read their numeric NBT from any numeric tag, as vanilla's do | `c40c92e0bc85b3d09d9d4bf262c25ae180865bd0` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | `crates/horizon-client/src/game/minecarts.rs`<br>`tools/command_exec_corpus.py` | does not run (render built as a dependency only; tests: horizon-client) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |

### 13.4 Does not run the test locally: no workspace-Rust path (17)

| PR | Title | Head SHA | API base SHA | Merge base (behind main) | Changed paths | Local `pr_ready --quick` at head | After rebase onto `41f822b6` | GitHub PR CI |
|---|---|---|---|---|---|---|---|---|
| [#10042](https://github.com/horizon-rust/horizon/pull/10042) | HDR design note: serve shader API 0.1 until 0.2 is on by default | `02ba821bcfa910462006487ac937a722cb7afd7b` | `d545e9df609b92f548f2a530b536daf8e7477059` | `d545e9df609b` (786) | `docs/api/shaders.md`<br>`docs/design/hdr-linear-light.md` | does not run (no workspace Rust path: cargo test steps skipped) | same | does not run (docs-only: Rust steps skipped); head's `verify`: success |
| [#10166](https://github.com/horizon-rust/horizon/pull/10166) | Launcher: a server on another computer, run over SSH | `cc474268e799683256f0cf986521a60a0be2e47c` | `f2f95dad8a1a701424a08bd989168297a7172b36` | `fb6121bce9fa` (798) | 4 files: `launcher`×4 | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10529](https://github.com/horizon-rust/horizon/pull/10529) | fix(launcher): support responsive wrapping on narrow windows | `052d9ebf20a9841495e6e1a09b326b774ed8bb8a` | `28a389b7a37373598391d8e18d87846bc4218e41` | `a1ff75a4a525` (21) | 9 files: `launcher`×9 | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10540](https://github.com/horizon-rust/horizon/pull/10540) | Keep exact changed paths and rename sources in tooling checks | `97f85f45308881642b3db3364b60e632fc77fc8f` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | `tests/test_pr_ready.py`<br>`tools/parity.py`<br>`tools/pr_ready.py` | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11042](https://github.com/horizon-rust/horizon/pull/11042) | Launcher: update status and Check now move from About to Updates | `6f2b33ff95a4fe3499b1ba53becfa272d80a2b5d` | `a1ff75a4a525b6a9e1ed68f74cf9d54161596370` | `a1ff75a4a525` (21) | `launcher/src/glass_settings.rs`<br>`launcher/src/main.rs` | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11064](https://github.com/horizon-rust/horizon/pull/11064) | Roadmap: attach 22 proposal issues to their items, two new Later items | `25ce3945c85f1ff29ec8e6f8880dc848d42249a3` | `28a389b7a37373598391d8e18d87846bc4218e41` | `a1ff75a4a525` (21) | `ROADMAP.md`<br>`roadmap/roadmap.json`<br>`roadmap/roadmap.toml` | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11079](https://github.com/horizon-rust/horizon/pull/11079) | Native mods on Intel Macs: the launcher brings Rust for x86_64 macOS | `cf20ccc4f9bcdb688d79bb64995f01b327d3bbdf` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `a1ff75a4a525` (21) | `docs/native-mods.md`<br>`launcher/src/rust_toolchain.rs` | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11087](https://github.com/horizon-rust/horizon/pull/11087) | Hosting: an AMP template for horizon-server | `594552fec4d8543e12578045f8b3fb3577685da7` | `398da2168910f1e234eb1d4cf5f37790037f2d32` | `398da2168910` (8) | 7 files: `docs`×1, `packaging`×5, `tests`×1 | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11089](https://github.com/horizon-rust/horizon/pull/11089) | README: a Performance section from the clean Horizon vs vanilla runs | `c81dc55a1c100e18ead3fa340c3bfa85f2d5ad00` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | `README.md` | does not run (no workspace Rust path: cargo test steps skipped) | same | does not run (docs-only: Rust steps skipped); head's `verify`: success |
| [#11096](https://github.com/horizon-rust/horizon/pull/11096) | Mod browser: a native mod's fit in its status, and a warning before installing one that won't build | `8fefaaf87c749dbfdddf2870f449b83fa83a08a8` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | 92 files: `launcher`×92 | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11112](https://github.com/horizon-rust/horizon/pull/11112) | use_item_on's oracle posts item_interact_start when a use starts | `f7af3f420fcb9529da4dd8c4ff96b221f753c2b6` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaDispenserOracle.java` | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11131](https://github.com/horizon-rust/horizon/pull/11131) | tests/test_harness_args: match system path separator for default work dir | `03d9b39fc41750b31ac2917173f34082f239b013` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | `tests/test_harness_args.py` | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11134](https://github.com/horizon-rust/horizon/pull/11134) | docs/server-levels: remove stale End credits for server players note | `8f39c8afd2dade09f05c4ed726e0bc4ae06e4ada` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | `docs/server-levels.md` | does not run (no workspace Rust path: cargo test steps skipped) | same | does not run (docs-only: Rust steps skipped); head's `verify`: success |
| [#11142](https://github.com/horizon-rust/horizon/pull/11142) | Let dolphin and spell parity harnesses use runner defaults | `a98ba02730df9e6f614082feae51429318290ffc` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | 4 files: `tests`×2, `tools`×2 | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11149](https://github.com/horizon-rust/horizon/pull/11149) | docs/native-client: update stale platform scope note | `bfdff7afb64dabc7d91dbf8110c02de997717a96` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | `docs/native-client.md` | does not run (no workspace Rust path: cargo test steps skipped) | same | does not run (docs-only: Rust steps skipped); head's `verify`: success |
| [#11151](https://github.com/horizon-rust/horizon/pull/11151) | Run the mounted player tick in the gateway parity scene | `ff416a1889d4908ed3e8b22be4afac20629d1edb` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | `tools/gateway_passenger_corpus.py` | does not run (no workspace Rust path: cargo test steps skipped) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#11156](https://github.com/horizon-rust/horizon/pull/11156) | docs/README.md: index live mod reloading design note | `4e36ae1e4ecdf3e3cc39801986ea633c24dd02b1` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `398da2168910` (8) | `docs/README.md` | does not run (no workspace Rust path: cargo test steps skipped) | same | does not run (docs-only: Rust steps skipped); head's `verify`: success |

### 13.5 Contrast only: #10194, #10195 (`unstable`)

| PR | Title | Head SHA | API base SHA | Merge base (behind main) | Changed paths | Local `pr_ready --quick` at head | After rebase onto `41f822b6` | GitHub PR CI |
|---|---|---|---|---|---|---|---|---|
| [#10194](https://github.com/horizon-rust/horizon/pull/10194) | Native mods: block state facts and the client's resource files, for maps and radars | `6d9ed21235472fde03acc093e31804dc206eb560` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | 8 files: `crates/horizon-client`×3, `crates/horizon-client-render`×3, `docs`×1, `tools/native-hook-fragments`×1 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |
| [#10195](https://github.com/horizon-rust/horizon/pull/10195) | Native mods: the client's entity types, textures, dropped items and skins | `cd86e8f45d778142c30e2198f0606c2f45789d34` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` | `41f822b65978` (0) | 10 files: `crates/horizon-client`×4, `crates/horizon-client-render`×3, `docs`×1, `tools/native-hook-fragments`×2 | **RUNS test, `audio` ON** (tests: horizon-client, horizon-client-lib, horizon-client-render, horizon-mod-api-gen) | same | **RUNS** (Linux, `--workspace`, `audio` ON); head's `verify`: success |

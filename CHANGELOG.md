# Changelog

All notable changes to Quiro are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow
[Semantic Versioning](https://semver.org/).

`scripts/version.mjs` moves everything under **[Unreleased]** into a dated
version section when you cut a release, so keep adding entries there as you land
changes.

## [Unreleased]

### Added

- Clicking or dragging on the editor timeline while a video is playing now
  jumps there and keeps playing, instead of being ignored until you pause.

### Changed

- On Windows, when the camera won't start because another app has it, the
  error now names that app (for example, "The camera is in use by Firefox").
- Messages outside the editors (the launch window, the recordings list, and
  Settings) now show as system notifications instead of in-window toasts,
  which got cut off in small windows. They fall back to toasts when
  notifications are turned off in Settings.

### Fixed

- Playing from one clip into the next no longer freezes the second clip on its
  first frame for up to three seconds (and makes zooms over it appear to
  shake). This happened when the second clip had already been previewed
  further in; its decoder kept reading from that later point and could not
  return the earlier frames playback asked for.
- The microphone and camera are released when a recording ends and the
  editor opens, so the system's in-use indicators turn off. They reconnect
  when you return to the main window. Opening Settings now releases the
  microphone too, not just the camera.

## [0.2.3] - 2026-10-13

### Fixed

- Choosing a microphone is near-instant on devices with slow format probing,
  such as Intel Smart Sound mics, instead of stalling for 30 seconds or more.
  Quiro no longer tries every audio format on every input just to list
  microphones, only probes the selected one, and does that off the
  microphone's message loop. The background device watcher stops
  continuously re-probing the microphone, too.

## [0.2.2] - 2026-10-13

### Fixed

- Starting a recording soon after choosing a microphone no longer fails with
  "microphone actor not running". A late stream-ready event reached the
  microphone after it locked for recording and stopped it. Starting a
  recording now also restarts the microphone if it has stopped, instead of
  failing until you pick the microphone again.

## [0.2.1] - 2026-10-11

- Maintenance release.

## [0.2.0] - 2026-09-24

### Added

- Desktop error tracking through PostHog for camera setup and frame conversion,
  device enumeration, capture thumbnails, window operations, and unexpected
  process termination. Camera reports include the failure stage, operating
  system, app version, requested format, and hardware vendor/product IDs without
  sending device serial numbers.
- Windows support manifests Common Controls v6 for native UI compatibility.
- Import videos from the editor's Clips panel and append them to the timeline.

### Changed

- Camera initialization now reports whether a device timed out before producing
  frames or produced frames that Quiro could not decode. Windows camera selection
  prefers stable device IDs and falls back to model IDs.
- Updating from Settings now downloads and installs the update, then relaunches
  Quiro automatically.
- Refined desktop launch toolbar sizing, navigation, and window close behavior.
- Recording sync checks now identify decoded source frames and tolerate dropped
  frames and runner scheduling delays.
- Renamed Rust example binaries to crate-specific names so workspace examples no
  longer collide.

### Fixed

- Windows Media Foundation camera enumeration now retains the DirectShow device
  when Media Foundation exposes no usable formats.
- Removed the Dependabot configuration to stop its scheduled CI activity.
- Custom window close buttons now close settings, screenshot editor, and other desktop windows.

## [0.1.5] - 2026-09-19

### Added

- Anonymous install/launch analytics for the desktop app (PostHog), plus
  download-button and pageview tracking on the website (Vercel Analytics).
  Both are disclosed alongside the app's existing crash reporting. The
  desktop app sends nothing unless a build carries a PostHog project key;
  `pnpm dev` now loads one from a local `.env` automatically.

### Changed

- Camera preview window now defaults to a larger 300px size (was 230px).

## [0.1.4] - 2026-09-16

### Added

- A new icon-tiles toolbar layout for the launch window, now the default.
  It shares recording settings, capture targets, device menus, and capture
  commands with the classic launch window. Press Ctrl+Shift+L
  (Cmd+Shift+L on macOS) to switch layouts, or launch with
  `--launch-window=toolbar` / `--launch-window=classic`; the choice is
  remembered for next time. See `apps/desktop/LAUNCH-WINDOWS.md`.
- Linux downloads and in-app updates: the website's download page now offers
  an AppImage alongside Windows/macOS, and the release pipeline publishes it
  to R2 so the desktop updater can serve it too.
- The in-app changelog now fetches from quiro.app first (so it can show
  what's new before you update), falling back to the bundled copy offline.
- Website: an interactive capture-mode toggle in the Capture section preview.

### Fixed

- Windows: the launch window's undecorated frame showed DWM's default
  rounded corners, a 1px accent border, and an opaque WebView2 background,
  all fighting the app's own CSS chrome. All three are now disabled for the
  toolbar layout.
- `pnpm release` reformatted every JSON version file through
  `JSON.stringify`, which disagreed with Biome's formatter and broke CI's
  lint job on release commits (as it did for v0.1.3). It now does a surgical
  version-field replace instead, like the existing Cargo.toml/Cargo.lock bump.

## [0.1.3] - 2026-09-13

- Maintenance release.

## [0.1.2] - 2026-09-10

- Maintenance release.

## [0.1.1] - 2026-09-09

### Added

- Release engineering: reproducible multi-platform bundling, `pnpm release`
  version tooling, CI on every push/PR, and tag-triggered release + nightly
  workflows with Tauri auto-update manifests published to Cloudflare R2.

### Fixed

- Installed Windows app failed to start: the FFmpeg 7.1 runtime DLLs
  (`avcodec-61`, `avformat-61`, …) were never bundled. They now ship beside the
  executable.
- Launch window crashed with "Cannot read properties of null (reading
  'variant')" after choosing a capture target, when a stale persisted
  `captureTarget: null` was loaded. Null is now coalesced to the default at
  every load path.

## [0.1.0] - 2025-09-01

### Added

- Initial internal build: screen + camera recording, studio editor,
  screenshot editor with arrow / shape / text / mask / focus annotations.

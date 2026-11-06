# Sound-device test hang evidence

Local `python3 tools/pr_ready.py --quick` can exit 1 because `horizon-client-render`'s `sound_device_button_in_sound_page_and_cycles` hangs, not because of a problem in https://github.com/horizon-rust/horizon/pull/10194 or https://github.com/horizon-rust/horizon/pull/10195.

## Conclusion

This hang is not a bug in #10194 or #10195. Those PRs do not touch this test or the client audio/render code that the test calls. It is also not a machine-wide CoreAudio deadlock: `coreaudiod` was running, and `system_profiler SPAudioDataType` still returned a device list in under 25 seconds.

## Stack evidence

On the first stuck `pr_ready` run, pid 65599 (`horizon_client_render-…`) was alive and idle at 0% CPU for about 56 minutes. A stack sample showed the thread parked inside CoreAudio, not executing Horizon render or audio logic: it had handed off to the OS audio layer (`AudioComponentInstanceNew` → a CoreAudio HAL plugin → a `mach_msg` wait) and was getting nothing back. The Horizon/cpal frames on the stack were only the callers that entered that OS call (`sound_device_button_in_sound_page_and_cycles` → `Opt::SoundDevice.cycle` → `audio::output_devices` → cpal `supports_output` / `supported_output_configs`).

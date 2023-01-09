# VRInsight CDU II MSFS Driver

## How to run
Download the latest binary and `keymap.yaml` from [_Releases_](https://github.com/callebstrom/vrinsight-cdu-ii-msfs-driver/releases). You can place these two files anywhere after downloading but note that `keymap.yaml` needs to be co-located with the binary.

## What

This driver connects directly to the VRInsight CDU II and translates its input to the appropriate HTML-events and LVARs for MSFS, enabling you control practically any MSFS FMS/CDU/MCDU using your VRInsight CDU II device. These are then forwarded either directly via SimConnect or via the [MobiFlight Event WASM module](https://github.com/Mobiflight/MobiFlight-Connector/).

## Why

In order to use the VRInsight CDU II device together with MSFS you need to run FSUIPC and LINDA (as per these [instructions](https://www.avsim.com/forums/topic/583434-linda-415-msfs-2020-compatible-5-jun-2022)).

While the above solution works, I've found it to be working half of the time at best and is heavily relying on the startup and execution order of LINDA, FSUIPC and their respective lua scripts order.

## Prerequisites
- MobiFlight WASM module (see [mobiflight-event-module](./lib/mobiflight-event-module/))

## Alternatives
- LINDA + FSUIPC (as described in the [_Why_](##Why) section)
- Serial2FP (no MSFS support AFAIK)

## TODO
- [ ] Installer
  - Register as MSFS startup binary
  - Install binary in appropriate dir
  - Install `mobiflight-event-module` if not already installed

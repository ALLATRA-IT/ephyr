Ephyr changelog
===============

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.3.6] · 2020-10-04
[0.3.6]: /../../tree/v0.3.6

[Diff](https://github.com/ALLATRA-IT/ephyr/compare/v0.3.5..v0.3.6)

### Added

- [VOD] meta info HTTP server:
    - Endpoints:
        - `PUT /`, `PUT /{playlist}`:
            - Optional `dry_run` URL query parameter to validate playlist without changing it.




## [0.3.5] · 2020-09-17
[0.3.5]: /../../tree/v0.3.5

[Diff](https://github.com/ALLATRA-IT/ephyr/compare/v0.3.4..v0.3.5)

### Added

- [VOD] meta info HTTP server:
    - Endpoints:
        - `PUT /`, `PUT /{playlist}`:
            - Optional `resolution` parameter of playlist.




## [0.3.4] · 2020-09-15
[0.3.4]: /../../tree/v0.3.4

[Diff](https://github.com/ALLATRA-IT/ephyr/compare/v0.3.3..v0.3.4)

### Fixed

- [VOD] meta info HTTP server:
    - Endpoints:
        - `GET /{proto}/{playlist}/{file}`:
            - Increase delay drift to 1 minute.
    - Background:
        - Broken playlist refilling with cached videos due to playback protection. 




## [0.3.3] · 2020-09-14
[0.3.3]: /../../tree/v0.3.3

[Diff](https://github.com/ALLATRA-IT/ephyr/compare/v0.3.2..v0.3.3)

### Added

- [VOD] meta info HTTP server:
    - Endpoints:
        - `GET /`: displays the whole current state of server; 
        - `GET /{playlist}`: displays the current state of a single playlist;
        - `PUT /{playlist}`: renews meta information of a singe playlist (authorized);
        - `DELETE /{playlist}`: removes of a single playlist (authorized);
        - `PUT /`:
            - Optional `segment_duration` parameter of playlist.
    - Background:
        - Renewing playlists initial positions every minute.
### Fixed

- [VOD] meta info HTTP server:
    - Endpoints:
        - `GET /{proto}/{playlist}/{file}`:
            - Inappropriate meta information for [`kaltura/nginx-vod-module`] resulting in broken playback on client.
        - `PUT /`:
            - Accepting clip durations non-aligned with segment duration;
            - Accepting days without clips.




## [0.3.2] · 2020-08-27
[0.3.2]: /../../tree/v0.3.2

[Diff](https://github.com/ALLATRA-IT/ephyr/compare/v0.3.1..v0.3.2)

### Fixed

- [VOD] meta info HTTP server:
    - Endpoints:
        - `PUT /`:
            - Inability to parse information from [allatra.video] API about videos with duration less than hour.




## [0.3.1] · 2020-08-26
[0.3.1]: /../../tree/v0.3.1

[Diff](https://github.com/ALLATRA-IT/ephyr/compare/v0.3.0..v0.3.1)

### Added

- CLI:
    - `ephyr serve vod-meta`:
        - `--request-max-size` option to configure maximum allowed size of the JSON body accepted by `PUT` HTTP request which renews [VOD] meta information.

### Fixed

- [VOD] meta info HTTP server:
    - Endpoints:
        - `PUT /`:
            - Missing response body when request JSON fails to parse. 




## [0.3.0] · 2020-08-01
[0.3.0]: /../../tree/v0.3.0

[Diff](https://github.com/ALLATRA-IT/ephyr/compare/v0.2.0..v0.3.0)

### BC Breaks

- CLI:
    - Default mixing `ephyr` command moved to `ephyr mix` sub-command.

### Added

- CLI:
    - `ephyr serve vod-meta` sub-command to run [VOD] meta info HTTP server.
- [VOD] meta info HTTP server:
    - Endpoints:
        - `GET /{proto}/{playlist}/{file}`: prepares meta information for [`kaltura/nginx-vod-module`];
        - `PUT /`: renews meta information (authorized).
    - Background:
        - Downloading [VOD] files to `--cache-dir`;
        - Synchronization of meta information with [VOD] files cache every 10 seconds.




## [0.2.0] · 2020-07-11
[0.2.0]: /../../tree/v0.2.0

[Diff](https://github.com/ALLATRA-IT/ephyr/compare/v0.1.0..v0.2.0)

### BC Breaks

- `teamspeak::Input` now produces a constant 48kHz sample rate;
- `silence::Filter` is removed (`teamspeak::Input` produces silence samples itself, when there is no audio in a [TeamSpeak] channel).

### Changed

- `ffmpeg::Mixer` now re-samples [RTMP] stream's audio to 48kHz _before_ mixing with [TeamSpeak] audio.

### Improved

- `teamspeak::Input`:
    - Emit 2-channels stereo audio ([#2]);
    - Use [Opus] FEC (forward error correction) ([#3]).
    
[#2]: /../../issues/2
[#3]: /../../issues/3




## [0.1.0] · 2020-07-04
[0.1.0]: /../../tree/v0.1.0

### Implemented

- `teamspeak::Input`:
    - Capturing audio from a [TeamSpeak] channel;
    - Mixing audio streams of multiple talkers.
- `silence::Filter`:
    - Filling audio stream with a silence if it produces no data.
- `ffmpeg::Mixer`:
    - Mixing STDIN audio stream with [RTMP] stream;
    - Correcting STDIN audio stream sample rate;
    - Delaying audio stream;
    - On-fly toggling of audio stream volume via [ZeroMQ].
- Mixing `Spec`:
    - Multiple mixing schemes for a single [RTMP] stream;
    - Multiple outputs for a single mixing result.
- [Docker] image:
    - [FFmpeg] 4.3 for mixing;
    - [SRS] 3.0 media server for receiving [RTMP] stream and optionally serving the mixing result.





[allatra.video]: https://allatra.video/
[Docker]: https://www.docker.com
[FFmpeg]: https://ffmpeg.org
[Opus]: https://opus-codec.org
[RTMP]: https://en.wikipedia.org/wiki/Real-Time_Messaging_Protocol
[Semantic Versioning 2.0.0]: https://semver.org
[SRS]: https://ossrs.net
[TeamSpeak]: https://teamspeak.com
[VOD]: https://en.wikipedia.org/wiki/Video_on_demand
[ZeroMQ]: https://zeromq.org

[`kaltura/nginx-vod-module`]: https://github.com/kaltura/nginx-vod-module

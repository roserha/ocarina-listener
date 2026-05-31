# REQ_TRACK

## Audio and Notes
| ID | Status | Notes |
|----|--------|-------|
| A1 | Met | Notes labeled as e.g. "D4" using CDEFGAB + octave |
| A2 | Not Met | Notekeeper cap is 12, requirement says 8 |
| A3 | Met | Mode over 30 samples, majority-non-silence check in place |
| A4 | Not Addressed | Volume threshold and sample count are hardcoded; no settings JSON exists |
| A5 | Met | Notes only added when interpretation detects a change from last note |
| A6 | Met | Silence resets last_note, preventing duplicate continuous entries |
| A7 | Not Addressed | No interpretation progress, confidence, or state indicator in UI |

## Songs
| ID | Status | Notes |
|----|--------|-------|
| S1 | Met | songbook.json exists and is loaded at startup |
| S2 | Met | check_played_notes tests ±1 octave variants |
| S3 | Met | All current songs are 4–7 notes; no code enforcement |
| S4 | Met | Song title is pushed to notekeeper on recognition |
| S5 | Not Addressed | No audio playback implemented; no audio data in songbook |

## Performance
| ID | Status | Notes |
|----|--------|-------|
| P1 | Not Addressed | Not benchmarked on RPi Zero 2W |
| P2 | In Progress | Songbook pre-loaded; blocked on A4 (settings JSON not yet implemented) |

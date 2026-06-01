# AUDIO AND NOTES
* **A1** - notes should be kept track of by labeling it by its pitch in CDEFGAB scale, and its octave
* **A2** - played notes should be kept in a dequeue structure, heretoafter called the notekeeper, wherein new notes come in, and after 8 notes, the oldest played note will get discarded
* **A3** - notes should only be added to the notekeeper if the program is confident that the user played the specific note - a process heretoafter labeled interpreting the note. this confidence shall be ascertained by getting the mode of the estimated note over a collection of note samples through which the majority is not silence. 
* **A4** - the user must be able to finetune certain parameters of this algorithm by changing an internal json file that lists the threshold of volume to differentiate between silence and notes, and how many note samples it takes to interpret said note, to easily fine-tune their settings dependent on their setup and instrument
* **A5** - notes should only be added to the notekeeper if the interpretation is successful in determining that a new note was indeed played - whether it be a different pitch or not.
* **A6** - the program shall never add multiple instances of the same continuous note. for the user to add two entries of the same exact pitch, there must be a registered period of silence.
* **A7** - the user must be made aware when the program is still interpreting the note that the user is playing, and how confident it is settings, and when the program is awaiting a new note.

# SONGS  
* **S1** - songs should be explicitly named and kept track of on a json file heretoafter called the songbook
* **S2** - songs in the songbook shall be recognizable by the program independent of octave played
* **S3** - songs should have between 3 and 8 notes, inclusive on both bounds
* **S4** - songs must always have a title for the notekeeper to display
* **S5** - when a song is recognized by the notekeeper, an audio snipped of said song shall be played, dependent on if that information is included for the recognized song within the songbook

# PERFORMANCE, DEVICE & PROJECT
* **P1** - program shall not take more than 100ms to have a single note sample for the notekeeper when being ran on a raspberry pi zero 2w
* **P2** - external variables and songbook songs must be loaded before the program readies its execution, as to not bog down processing time during main functionalities
* **P3** - project must be able to have a service installed on its host machine where the program automatically starts on boot
* **P4** - project must be able to generate an os image where the service, program and i2c microphone firmware configurations are already installed

# DISPLAY
* **D1** - program must display on the 16 segment display the 3 most recent notes on the top row, alongside a signalizer for current state, and the most recent song name on the bottom. 
* **D2** - if no song was played, display "Ocarina🎵Listener" instead of a song title. 
* **D3** - if less than three notes were played, still keep signalizer on columns 14-16.


(
cd ./ocarina-os/meta-ocarina/recipes-ocarina/ocarina-listener/files/sounds || exit 1

for f in *.wav; do
    peak=$(ffmpeg -nostdin -i "$f" -af volumedetect -f null - 2>&1 \
        | grep max_volume \
        | sed 's/.*max_volume: //; s/ dB//')

    if [ -z "$peak" ]; then
        echo "SKIP $f (could not measure peak)"
        continue
    fi

    gain=$(awk "BEGIN{print -1 - $peak}")
    echo "$f: peak=${peak}dB, applying ${gain}dB"

    ffmpeg -nostdin -y -i "$f" -filter:a "volume=${gain}dB" "/tmp/norm_$f"
    mv "/tmp/norm_$f" "$f"
done
)

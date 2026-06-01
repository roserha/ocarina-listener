./scripts/build-off-target.sh && ssh pi@$(cat ./scripts/secrets/RPIIP.txt) "./ocarina-listener"

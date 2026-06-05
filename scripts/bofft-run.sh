./scripts/build-off-target.sh && ssh root@$(cat ./scripts/secrets/RPIIP.txt) "sudo reboot"

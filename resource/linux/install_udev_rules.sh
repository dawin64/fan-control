#! /bin/bash

STEAMOS=0
STEAMOS_READONLY=0

# Test for SteamOS and disable readonly mode if we're running on it
if command -v steamos-readonly >& /dev/null
then
	# Test if SteamOS readonly mode is enabled
	if sudo steamos-readonly status | grep 'enabled'
	then
		echo "steamos readonly mode is true"
		STEAMOS_READONLY=1
	fi

	STEAMOS=1
	sudo steamos-readonly disable
fi

# Download udev rules file
#wget https://raw.githubusercontent.com/wiiznokes/fan-control/flatpak/resource/linux/60-fan-control.rules

# Move udev rules file to udev rules directory
sudo mv 60-fan-control.rules /usr/lib/udev/rules.d
#sudo install -m644 90-fan-control.rules /usr/lib/udev/rules.d

# Reload the rules
sudo udevadm control --reload-rules && sudo udevadm trigger


if [ "$STEAMOS" = 1 ] ; then
	if [ "$STEAMOS_READONLY" = 1 ] ; then
		sudo steamos-readonly enable
	fi
fi

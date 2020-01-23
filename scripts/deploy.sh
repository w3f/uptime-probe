#!/bin/sh

/scripts/deploy.sh -t helm -a "--set image.tag=${CIRCLE_TAG} uptime-probe w3f/uptime-probe"

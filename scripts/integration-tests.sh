#!/bin/bash

source /scripts/common.sh
source /scripts/bootstrap-helm.sh


run_tests() {
    echo Running tests...

    wait_pod_ready uptime-probe
}

teardown() {
    helm delete uptime-probe
}

main(){
    if [ -z "$KEEP_W3F_UPTIME_PROBE" ]; then
        trap teardown EXIT
    fi

    /scripts/build-helm.sh \
        --set environment=ci \
        --set origin=test \
        --set image.tag="${CIRCLE_SHA1}" \
        uptime-probe \
        ./charts/uptime-probe

    run_tests
}

main

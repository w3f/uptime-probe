[![CircleCI](https://circleci.com/gh/w3f/uptime-probe.svg?style=svg)](https://circleci.com/gh/w3f/uptime-probe)

# Uptime Probe

This probe checks a set of websites and exposes the results as prometheus
metrics. These metrics are used on an alert that triggers if any of the
configured sites can't be accessed by the probe.

## Configuration

The binary created with `cargo build` accepts one single argument, the path to a
configuration file which defines the sites to check. An example is provided in
the repo [here](cfg.sample.yaml).

When the binary is deployed to the kubernetes production cluster it uses
[this configmap](charts/uptime-probe/templates/configmap.yaml) for getting the
configuration settings.


## Files

These are the main directories in the repo:

* `charts`: contains the uptime-probe helm chart, besides the deployment and
service resources it contains manifests for these custom resources:

  * `servicemonitor`: allows to scrape metrics from uptime-probe.

  * `configmap`: basic configuration of the probe, including the set of websites
  to ccheck.

* `.circleci`: defines the CI/CD configuration.

* `scripts`: contains:

  * `integration-tests.sh`: automated checks to verify that the components can
  be properly deployed.

  * `deploy.sh`: commands to release the application to the production cluster
  using the published chart.

* `src, Cargo.*`: code for uptime-probe and additional Rust project files.

* `Dockerfile`: definition of the image used to deploy the probe. It consists on
a multi-stage DDocker build, with a first stage that generates a static binary
from Rust source files and a second stage which puts this binary on an Alpine
image, resulting in a very lightweight final image (~7Mb).

## Environment variables

In order to be able to deploy to production, these environment variables must be
available:

* `$DIGITALOCEAN_ACCESS_TOKEN`

* `$GITHUB_BOT_TOKEN`

* `$DOCKER_USER`

* `$DOCKER_PASSWORD`

These values are already set on CI, and are available on 1Password, under the
Infrastructure vault, the GitHub bot token in an item called `GitHub bot`, the
Docker credentials in an item called `Docker Hub Bot` and the Digital Ocean
access token in the `DigitalOcean API credentials` item.

## CI/CD Workflow

When a PR is proposed to this repo, the integration tests defined by
`scripts/integration-tests.sh` are executed on a Kubernetes cluster created on
CI using the code from the PR, currently they just check that the component can
be deployed and deleted without errors.

After the PR is merged into master, when a semantic version tag (`vX.Y.Z`) is
pushed the tests are run again and, if all is ok, the chart is published and the
probe is deployed to production. Note that the tag version pushed must match the
version in [./charts/uptime-probe/Chart.yaml]()

## Running tests

Tests can be run and debugged locally, you need to have [docker](https://docs.docker.com/install/)
and [CircleCI CLI](https://circleci.com/docs/2.0/local-cli/) installed, then run:
```
$ circleci local execute --job cargoTests
```
for Cargo tests and:
```
$ circleci local execute --job integrationTests
```
for e2e integration tests (involving k8s component deployment).
test

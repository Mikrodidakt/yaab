# Introduction

By default, Bakery utilizes Docker and relies on the yaab-workspace image. Presently, Bakery has minimal requirements, only necessitating the availability of Docker on the system and that the user invoking Bakery is a member of the docker group. It's important to note that Bakery should never run as root. If you encounter permission issues when running Bakery with Docker, it's likely that the user is not part of the docker group.

# Setup Docker

There are multiple ways to install docker on your system and yaab has currently no preference. For reference on how to setup docker please see [setup-docker.sh](https://github.com/Mikrodidakt/yaab/blob/main/scripts/setup-docker.sh). This script currently supports setting up Docker on ubuntu and debian it is based on [Install Docker](https://docs.docker.com/engine/install/).

## Docker Group

Installing and correctly configuring Docker is a prerequisite for Bakery. It's essential that each user is a member of the docker group to avoid running Docker as root. For detailed instructions on [post-installation](https://docs.docker.com/engine/install/linux-postinstall/) steps on Linux, please refer to Post Install on Linux. Once Docker is set up, ensure that the user belongs to the docker group by running:


```bash
user@node:/dir$ groups
user adm cdrom sudo dip plugdev docker
```

# Bakery Workspace Image

The yaab workspace image can be pulled from Github Container Registry by running

```bash
user@node:/dir$ YAAB_VERSION=$(yaab --version)
user@node:/dir$ YAAB_VERSION=${YAAB_VERSION##* }
user@node:/dir$ docker pull ghcr.io/mikrodidakt/yaab/yaab-workspace:${YAAB_VERSION}
```

Opening a shell to the yaab-workspace can be done by running

```bash
user@node:/dir$ docker run -it ghcr.io/mikrodidakt/yaab/yaab-workspace:${YAAB_VERSION} /bin/bash
```

## Custom Worksapce Image

If the default yaab workspace image is not enough a custome image can easily be setup. To get yaab to use the custom image change the values in the workspace config file

```json
        "docker": {
                "registry": "registry.io",
                "image": "custom-workspace",
                "tag": "x.y.z",
                "args": [
                ]
        }
```

When creating the custom yaab workspace make sure to include the following

```bash
RUN wget https://github.com/Mikrodidakt/yaab/releases/download/v${YAAB_VERSION}/yaab-v${YAAB_VERSION}.deb
RUN sudo dpkg -i yaab-v${YAAB_VERSION}.deb

# Setting up a yaab specific bash env pulled in by /etc/bash.bashrc 
RUN mkdir -p /etc/yaab && \
     echo "source /etc/yaab/yaab.bashrc" >> /etc/bash.bashrc
```

if the yaab workflow to use the yaab aliases in the [yaab shell](sub-commands.md) is desired.

## Bootstrap Yaab

When starting yaab the first step is that yaab will bootstrap its self into the yaab-workspace image. In the default yaab-workspace image yaab is installed but sometimes it is not desired to use the version inside the container. This can be accomplished by adding the following to the workspace config file

```json
        "docker": {
                "registry": "registry.io",
                "image": "custom-workspace",
                "tag": "x.y.z",
                "args": [
                    "-v /usr/bin/yaab:/usr/bin/yaab:ro"
                ]
        }
```

This will make sure that the external yaab version is used instead of the internal version.

# Crops

The Yocto Project has established a Docker project named Crops. Although Bakery does not currently utilize it, integrating Crops containers is a future objective. These images are structured differently, and while there hasn't been an opportunity to explore their integration with Bakery yet, it is a desirable goal. Utilizing Crops containers could enhance Bakery's functionality and efficiency.

* https://hub.docker.com/r/crops/poky
* https://github.com/crops/yocto-dockerfiles
* https://github.com/crops/poky-container


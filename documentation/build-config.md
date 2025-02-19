# Introduction

The build config is what describes building the product for yaab. A typical build config would have a similare format as bellow


```json
{
        "version": "5",
        "name": "",
        "product": "",
        "project": "",
        "description": "",
        "arch": "",
        "context": [
        ],
        "include": [
        ],
        "tasks": {
                "task": {
                        "index": "0",
                        "name": "task",
                        "initenv": "",
                        "description": "",
                        "disabled": "true|false",
                        "condition": "true|false",
                        "builddir": "",
                        "build": "",
                        "clean": "",
                        "artifacts": []
                }
        },
        "flash": {
                "cmd": ""
        },
        "deploy": {
                "cmd": ""
        },
        "upload": {
                "cmd": ""
        },
        "setup": {
                "cmd": ""
        },
        "sync": {
                "cmd": ""
        }
}
```

The build config can be split up in

* Config data   - configuration data specific for config like config file format version and config name which is currently the same as the product name.
* Product data  - product data like name, description, arch.
* Context data  - context variables that can be used throught the build config.
* Tasks data    - to build a product multiple tasks might be required. The tasks data contains a list of tasks and defines what each task should do.
  * Artifacts data - The artifacts data is part of the task data and contains what artifacts to collect for each task.
* Deploy data   - information on how to deploy an image to the target.
* Upload data   - information on how to upload firmware to a artifactory server.
* Setup data    - information on how to setup the workspace e.g. initialization of git submodules
* Sync data     - information on how to sync the workspace e.g. sync/update of git submodules

# Config Data

## version

The config version is used to mark breaking changes to the build config format currently it is at version 5. If the format changes the version will be bumped and yaab will allert that the build config format needs to be migrated to the new format.

## name, product & project

Normaly only the name is needed and the product and project will be set to match the name but sometimes the product and projects needs to be different and can then be set to a specific value that is different from the name of the build config.

# Product Data

## name

The same as the build config name currently.

## description

A short description of the product listed when running the [List](sub-commands.md#List) sub-command.

## arch

What arch the product is. Might not be used but can be used as a context variable in paths or scripts. It is normally part of the naming convention of the artifacts after built an image in bitbake.

# Context Data

The context data is a list of variables that can be used by the rest of the build config. To use a context variable in the build config declare it in the context list in the build config

```json
{
  "context": [
    "CTX_VAR=test"
  ]
}
```

This context variable can then be used in the build config by wrapping it inside

```json
{
  "bb": {
    "localconf": [
      "$#[CTX_VAR]"
    ]
}
```

Any context variable in the build config will be expanded when yaab is parsing the build config. The context is a concept that is made up of two context variable type "built-in" variables and "config" variables. The "config" context variables are the once defined in the context section of the build config while the "built-in" variables are comming from the yaab binary. The values of the "built-in" variables are either defined by the workspace.json or by a combination that the yaab binary will define in run-time. Currently the following "built-in" variables are avilable to be used in the build config

```
YAAB_ARCH
YAAB_PRODUCT_NAME
YAAB_BUILD_CONFIG
YAAB_BUILD_VARIANT
YAAB_ARTIFACTS_DIR
YAAB_LAYERS_DIR
YAAB_SCRIPTS_DIR
YAAB_BUILDS_DIR
YAAB_WORK_DIR
YAAB_PLATFORM_VERSION
YAAB_BUILD_ID
YAAB_PLATFORM_RELEASE
YAAB_BUILD_SHA
YAAB_BUILD_VARIANT
YAAB_RELEASE_BUILD
YAAB_ARCHIVER
YAAB_DEBUG_SYMBOLS
YAAB_DEVICE
YAAB_DATE
YAAB_TIME
```

To get the up to date list please refere to [BUILT_IN_CONTEXT_VARIABLES](https://github.com/Mikrodidakt/yaab/blob/main/src/data/context.rs#L13). Some of the "built-in" context variables will be exposed to the bitbake environment by getting included to the local.conf. To get a list of what context variables a build config offeres and the values of them run the [list](sub-commands.md#context) sub-command with --ctx flag.

## YAAB_DATE and YAAB_TIME

The YAAB_DATE and YAAB_TIME context variables will be expanded to the current date and time. Currently the format is hardcoded to YY-MM-DD and HH:MM but shortly locale should be used so the format is picked up from the system instead.

# Include Multuple Build Configs

There are cases where multiple product build configs are defined in a workspace where these product are using the same tasks and/or the custome sub-commands. Each product could have it's own specific context variables that the tasks and custome sub-commands. This will prevent duplication of build data in the build configs. The product build config will contain

```json
        "include": [
          "tasks"
          "subcommands"
        ],
```

Bakery will by default look for tasks.json and subcommands.json under includes dir in the configs dir defined in the workspace.json. If nothing is define in the workspace.json it will search for the included build configs under

```bash
configs/include
```

Both the configs dir and include dir can be set in the workspace.json for more information please see [workspace config](workspace-config.md). The format of a included build config is the same as the product build config but it should only include the defined tasks and custom subcommands.

# Tasks Data

The tasks data contains a list of tasks needed to build a product.

```json
        "tasks": {
        },
```

## index

This is just to make sure that the tasks are executed in an expected order will most likely be removed in a later release.

## name

The name of the task should be unique

## type

A task can be a bitbake task or a non-bitbake task. The different types looks has a slightly different format. Default value is bitbake.

## disabled

Sometimes a task is needed but it should not be executed by default when not specifing a task and running a full build. For example a signing task that requires some additional resources like an HSM when signing so it should only be executed by a specific signing node then it can be disabled. It will then only be executed when the task is specificelly specified in the yaab command using the the task flag in the [build](sub-commands.md#Build).

## condition

Sometimes a task needs to only run under a specific condition. By default the condition is true but it is possible to use a [Context](build-config.md#context). For example yaab has the variant flag which will set the context variable $#[YAAB_RELEASE_BUILD] to one which can then be used as a condition to only execute a specific task.

```json
{
  "task": {
    "index": "0",
    "name": "task",
    "condition": "$#[YAAB_RELEASE_BUILD]",
    "build": "build.sh",
    "clean": "clean.sh",
    "artifacts": []
  }
}
```

### non-bitbake

```json
{
  "task2": {
    "index": "0",
    "type": "non-bitbake",
    "disabled": "false",
    "name": "",
    "builddir": "",
    "build": "",
    "clean": "",
    "artifacts": []
  }
}
```

#### build

Sometimes there are taskes needed to be executed after the image has been built. One such example is signing of firmware. Then a non-bitbake task can be defined.


```json
{
  "sign-image": {
    "index": "0",
    "type": "non-bitbake",
    "name": "sign-image",
    "disabled": "true",
    "builddir": "$#[SIGNING_DIR]",
    "build": "$#[YAAB_SCRIPTS_DIR]/sign.sh $#[YAAB_IMAGE]",
    "clean": "",
    "artifacts": []
  }
}
```

#### clean

The clean command is only used by the non-bitbake task it can be a shell command or a script that is called.


```json
{
  "sign-image": {
    "index": "0",
    "type": "non-bitbake",
    "name": "sign-image",
    "disabled": "true",
    "builddir": "$#[SIGNING_DIR]",
    "build": "$#[YAAB_SCRIPTS_DIR]/sign.sh $#[YAAB_IMAGE]",
    "clean": "$#[YAAB_SCRIPTS_DIR]/clean.sh $#[YAAB_IMAGE]",
    "artifacts": []
  }
}
```

#### builddir

The builddir is only used by the non-bitbake task and is used to change working directory before executing the build or clean command.

## artifacts

Each task has the capability to collect specific files. All collected files will be placed in the artifacts directory, which is defined in the workspace config. The artifacts directory is specified by the context variable YAAB_ARTIFACTS_DIR. I will refer to the artifacts directory using the context variable YAAB_ARTIFACTS_DIR.

Artifacts are organized as a list of children, where each child can have a type. If no type is specified, the default type "file" will be used.

### file

Collect file 'test/file1.txt' and copy it to 'YAAB_ARTIFACTS_DIR/file1.txt'.

```json
  "artifacts": [
        {
            "source": "test/file1.txt"
        },
        {
            "source": "test/file2.txt",
            "dest": "test/renamed-file2.txt"
        }
  ]
```

Rename 'test/file2.txt' to 'renamed-file2.txt' and copy it 'YAAB_ARTIFACTS_DIR/test/'.


### directory

Create a directory in the 'YAAB_ARTIFACTS_DIR' directory named 'dir' and copy all artifacts under 'YAAB_ARTIFACTS_DIR/dir/'

```json
  "artifacts": [
      {
          "type": "directory",
          "name": "dir",
          "artifacts": [
              {
                  "source": "file1.txt"
              },
              {
                  "source": "file2.txt",
                  "dest": "renamed-file2.txt"
              }
          ]
      }
  ]
```

### archive

Create a archive in the 'YAAB_ARTIFACTS_DIR' directory named 'test.zip' and collect the all artifacts in the archive

```json
  "artifacts": [
          "type": "archive",
          "name": "test.zip",
          "artifacts": [
              {
                  "source": "file1.txt",
                  "dest": "renamed-file2.txt"
              }
          ]
  ]
```

The archive type currently supports the following archives zip, tar.bz2 and tar.gz.

### manifest

Create a manifest file in the 'YAAB_ARTIFACTS_DIR' directory named 'test-manifest.json'. The manifest can contain build data.

```json
  "artifacts": [
        {
            "type": "manifest",
            "name": "test-manifest.json",
            "content": {
                "machine": "$#[YAAB_MACHINE]",
                "date": "$#{YAAB_DATE}",
                "time": "${YAAB_TIME}",
                "arch": "$#[YAAB_ARCH]",
                "sha": "$#[YAAB_BUILD_ID]",
                "variant": "$#[YAAB_BUILD_VARIANT]",
                "version": "$#[YAAB_PLATFORM_VERSION]"
            }
        }
  ]
```

### link

Create a symbolic link in the 'ARTIFACTS_DIR' directory named 'link.txt' pointing to 'test/file.txt'.

```json
  "artifacts": [
        {
            "type": "link",
            "name": "link.txt",
            "source": "test/file.txt"
        }
  ]
```

### conditional

Create a symbolic link in the 'ARTIFACTS_DIR' directory named 'link.txt' pointing to 'test/file.txt' if the 'condition' is true.

```json
  "artifacts": [
        {
            "type": "conditional",
            "condition": "$#[YAAB_ARCHIVER]",
            "artifacts": [
              {
                "type": "link",
                "name": "link.txt",
                "source": "test/file.txt"
              }
            ]
        }
  ]
```

The following conditions are interpreted as true

```bash
"1" | "yes" | "y" | "Y" | "true" | "YES" | "TRUE" | "True" | "Yes"
```

### Context

All context variables can be used in the artifacts the only place where context variables cannot be used is in the 'type' for the artifacts.

# Custom Sub-Commands

The custom sub-commands are to define sub-commands that is acting more like proxies so that yaab can be used as one tool for the entire work-flow when building, cleaning, deploying, uploading, setup and syncing. The custom sub-commands are likely the same for most products so it is recommended to use the context variables for product specific data and then use the context variables when calling the defined custom sub-command. The sub-commands can call either a script or a specific command. Each custom sub-command is also exposed in the yaab workspace shell for easy access.

## deploy

The deploy section currently is just made up of a cmd. This can be used to define a custom deploy command making use of the context variables. If not default a default echo command will be used

```json
"deploy": {
        "cmd": "$#[YAAB_SCRIPTS_DIR]/deploy.sh $#[YAAB_ARTIFACTS_DIR]/full-image-$#[YAAB_MACHINE].mender $#[YAAB_DEVICE]"
}
```

## upload

The upload section currently is just made up of a cmd. This can be used to define a custom upload command making use of the context variables.If not default a default echo command will be used

```json
"upload": {
        "cmd": "$#[YAAB_SCRIPTS_DIR]/upload.sh $#[YAAB_ARTIFACTS_DIR]/full-image-$#[YAAB_MACHINE].mender $#[MENDER_ARTIFACT_SERVER]"
}
```

## setup

The setup section currently is just made up of a cmd. This can be used to define a custom setup command making use of the context variables.If not default a default echo command will be used

```json
"setup": {
        "cmd": "$#[YAAB_SCRIPTS_DIR]/setup.sh"
}
```

## sync

The sync section currently is just made up of a cmd. This can be used to define a custom sync command making use of the context variables.If not default a default echo command will be used

```json
"sync": {
        "cmd": "$#[YAAB_SCRIPTS_DIR]/sync.sh"
}
```
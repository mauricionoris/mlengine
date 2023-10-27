#!/bin/bash

. /home/mnf/repo/rust/mlengine/mlengine_env/bin/activate

 python /home/mnf/repo/rust/mlengine/main.py $1 $2 $3

 deactivate

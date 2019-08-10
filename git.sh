#!/bin/bash

# define dir variable get workdir
dir=`pwd`

# in workdir
cd $dir
git pull
git add .

# 提示输入提交信息
echo -n "input commit message:"
read input_msg

# 开始提交代码
git commit -a -m "$input_msg"
git push

echo "git commit and push success"

# 添加文件后需要在终端给脚本文件执行权限
# chmod 777 git.sh

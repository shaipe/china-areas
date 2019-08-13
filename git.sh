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

# 1. 预览将要删除的文件
# git rm -r -n --cached 文件/文件夹名称 
# 加上 -n 这个参数，执行命令时，是不会删除任何文件，而是展示此命令要删除的文件列表预览。
# 2. 确定无误后删除文件
# git rm -r --cached 文件/文件夹名称
# 3. 提交到本地并推送到远程服务器
# git commit -m "提交说明"
# git push origin master
# 4. 修改本地 .gitignore 文件 并提交
#   git commit -m "提交说明"
#   git push origin master

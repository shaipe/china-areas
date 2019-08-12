dir=`pwd`
echo $dir;

# 中括号之间必须要有空格,否则会报错
if [ $1 == "linux" ];
then
    echo "start compile to linux app...";
    CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl
    # 设定编译后二进制文件生成目录
    dir=$dir"/target/x86_64-unknown-linux-musl/release"
else
    echo "start compile to mac app...";
    cargo build --release
    dir=$dir"/target/release"
fi

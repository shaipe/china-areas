中华人民共和国行政区划数据库
===

[![](https://img.shields.io/github/issues/shaipe/china-areas.svg)](https://github.com/shaipe/china-areas/issues) [![](https://img.shields.io/github/forks/shaipe/areas.svg)](https://github.com/shaipe/china-areas/network) [![](https://img.shields.io/github/stars/shaipe/china-areas.svg)](https://github.com/shaipe/china-areas/stargazers) [![](https://img.shields.io/github/release/shaipe/china-areas.svg)](https://github.com/shaipe/china-areas/releases)

中国行政区划代码，包括五级行政区划详细代码，县级以上区划地理围栏。

数据来自国家统计局、民政部、高德地图，均为公开数据。

## 数据来源

*   民政部、国家统计局：
    * [中华人民共和国行政区划代码，更新时间：2019-06-21](http://www.mca.gov.cn/article/sj/xzqh/2019) 
    * [中华人民共和国国家统计局-统计用区划和城乡划分代码, 更新时间：2019-01-31](http://www.stats.gov.cn/tjsj/tjbz/tjyqhdmhcxhfdm/)
    * [中华人民共和国国家统计局-统计用区划代码和城乡划分代码编制规则](http://www.stats.gov.cn/tjsj/tjbz/200911/t20091125_8667.html)

* 高德地图公开数据
    * 行政区划地理边界数据来自：[高德地图 行政区划](http://lbs.amap.com/api/webservice/guide/api/district)

* 京东地址公开数据
    * [京东接口数据](https://d.jd.com/area/get?fid=0)

## 使用说明

    本项目采用 `rust-lang` 编写, release 到bin目录的各平台中,直接可以使用工具更新最新的数据到本地

### 命令工具

```bash
# 进入二进制文件目录后运行如下命令
# 第一个数表示获取文件的格式
# 第二个参数表示获取行政区划的级次
./areas sql 3
```
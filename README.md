##前端解析mkv的落地实现

###rust代码
src中，主要是用到matroska-demuxer等库对mkv文件进行解析。然后配合wasm-bindgen等库编译为wasm供前端调用

###前端展示代码
仅做展示验证是否生效。开启，使用http-server开启本地服务器即可。

###后续
这只是一个rust落地小demo。但是基于rust+wasm帮助前端完成mkv文件的解析，是可落地生产并且优于使用@ffmpeg-wasm的（代码体积问题）。个人认为这是一个很棒的尝试。



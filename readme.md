# BevyでRustGPUを使いたい(WIP)
0.10.1では[spirvの読み込みに難がある](https://github.com/bevyengine/bevy/pull/7772)のでもう少し待つ。

[bevy-rust-gpu](https://github.com/Bevy-Rust-GPU/bevy-rust-gpu)もあるけどボイラープレート量がどうだろう。

Bevyはバックエンドにwgpuを使っている。
wgpuが実装する規格WebGPUはgeometryやtessellationには対応していない。
MeshShaderが来るかもしれないとか。
バックエンドを[Vulkano](https://github.com/vulkano-rs/vulkano)とかに[切り替え](https://github.com/hakolao/bevy_vulkano)た方が早いかも。


